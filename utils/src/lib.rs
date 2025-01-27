use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc, Datelike, Days};
use ics::{
    parameters,
    properties::{Description, DtEnd, DtStart, RRule, Summary},
    Event, ICalendar,
};
use itertools::Itertools;
use scraper::{Html, Selector};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Lesson {
    dtstart: NaiveDateTime,
    dtend: NaiveDateTime,
    repeat_until: Option<NaiveDateTime>,
    location: String,
    instructors: String,
}

#[derive(Debug, Clone, Default)]
pub struct Class {
    name: String,
    class_nbr: String,
    section: String,
    component: String,
    lessons: Vec<Lesson>,
}

// TODO: Replace Optional with result + errors
fn create_lesson(
    (days_and_times, location, instructors, start_and_end_dt): (String, String, String, String),
) -> Option<Lesson> {
    let (dstart, repeat_until) = match start_and_end_dt.split(" - ").map(|date| {
        NaiveDate::parse_from_str(date, "%d/%m/%Y").unwrap()
    }).collect::<Vec<_>>()[..] {
        [dstart, repeat_until, ..] => (dstart, repeat_until),
        _ => return None,
    };

    let day = &days_and_times[..2];
    let day: i32 = match day {
        "Mo" => 1,
        "Tu" => 2,
        "We" => 3,
        "Th" => 4,
        "Fr" => 5,
        "Sa" => 6,
        "Su" => 7,
        _ => return None,
    };

    let times = &days_and_times[3..];
    let (mut dtstart, mut dtend) = match times.split(" - ")
        .map(|time| {
            let time = NaiveTime::parse_from_str(time, "%I:%M%p").unwrap();
            dstart.and_time(time)
    }).collect_tuple() {
        Some((dtstart, dtend)) => (dtstart, dtend),
        _ => return None,
    };

    let dtstart_day = dtstart.weekday().number_from_monday() as i32;
    if dtstart_day != day {
        let mut offset = day - dtstart_day;
        if offset < 0 {
            offset += 7;
        }
        dtstart = dtstart + Days::new(offset as u64);
        dtend = dtend + Days::new(offset as u64);
    }

    let repeat_until = match repeat_until {
        repeat_until if repeat_until == dstart => None,
        repeat_until => repeat_until.and_hms_opt(23, 59, 59),
    };

    Some(Lesson {
        dtstart,
        dtend,
        repeat_until,
        location,
        instructors,
    })
}

#[must_use]
pub fn parse_classes(document: &str) -> Vec<Class> {
    let module_selector: Selector = Selector::parse(".ps_pagecontainer>.PSPAGECONTAINER .ps_pspagecontainer>.PSPAGECONTAINER>tbody>tr:nth-child(10) .PABACKGROUNDINVISIBLE>tbody>tr:nth-child(even) .PSGROUPBOXWBO").expect("Should be a valid css selector for finding modules");
    let title_selector = Selector::parse(".PAGROUPDIVIDER")
        .expect("Should be a valid css selector for finding titles");
    let date_selector = Selector::parse(
        ".PSGROUPBOX>tbody>tr:nth-child(3) .PSLEVEL3GRID>tbody>tr:not(:first-child)",
    )
    .expect("Should be a valid css selector for finding dates");
    let info_selector = Selector::parse("td")
        .expect("Should be a valid css selector for finding information of each date");

    let document = Html::parse_document(document);
    let result = document.select(&module_selector);
    result
        .filter_map(|x| -> Option<Class> {
            let name = match x.select(&title_selector).next() {
                Some(x) => x.inner_html(),
                None => String::default(),
            };
            let lessons = x.select(&date_selector).map(|x| {
                x.select(&info_selector).map(|x| {
                    x.text()
                        .collect::<String>()
                        .chars()
                        .filter(|x| !(x.eq(&'\n')))
                        .collect::<String>()
                })
            });
            let (class_nbr, section, component) = lessons
                .clone()
                .next()
                .and_then(|x| x.take(3).collect_tuple())
                .unwrap_or_default();
            let lessons: Vec<Lesson> = lessons
                .filter_map(|x| match x.skip(3).take(4).collect_tuple() {
                    Some(x) => create_lesson(x),
                    None => None,
                })
                .collect();
            if lessons.is_empty() {
                None
            } else {
                Some(Class {
                    name,
                    class_nbr,
                    section,
                    component,
                    lessons,
                })
            }
        })
        .collect()
}

fn generate_uid() -> String {
    format!("{}@SUTD-timetable", Uuid::new_v4())
}

fn format_dt(dt: NaiveDateTime) -> String {
    format!("{}", dt.format("%Y%m%dT%H%M%S"))
}

#[must_use]
pub fn create_ics(classes: &[Class]) -> ICalendar {
    let dtstamp = Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
    let mut cal = ICalendar::new("2.0", "-//SUTD//SUTD Timetable 1.0//EN");
    let events = classes.iter().flat_map(|class| {
        class.lessons.iter().map(|lesson| {
            let class_details = format!(
                "Class nbr.: {}\nSection: {}\nComponent: {}\n",
                class.class_nbr, class.section, class.component
            );
            let mut event = Event::new(generate_uid(), dtstamp.clone());
            event.push(Summary::new(&class.name));
            event.push(Description::new(format!(
                "Instructors: {}\nLocation: {}\n----\n{class_details}",
                lesson.instructors, lesson.location
            )));
            let mut dtstart = DtStart::new(format_dt(lesson.dtstart));
            dtstart.append(parameters!("TZID"=>"Asia/Singapore"));
            event.push(dtstart);
            let mut dtend = DtEnd::new(format_dt(lesson.dtend));
            dtend.append(parameters!("TZID"=>"Asia/Singapore"));
            event.push(dtend);
            if let Some(repeat_until) = lesson.repeat_until {
                event.push(RRule::new(format!(
                    "FREQ=WEEKLY;UNTIL={}",
                    format_dt(repeat_until)
                )));
            }
            event
        })
    });
    for event in events {
        cal.add_event(event);
    }
    cal
}
