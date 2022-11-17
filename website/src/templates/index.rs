use perseus::Template;
use sycamore::prelude::{view, Html, Scope, SsrNode, View};
use sycamore::rt::JsCast;
use wasm_bindgen::{closure::Closure, JsValue};

#[perseus::template_rx]
pub fn index_page<G: Html>(cx: Scope) -> View<G> {
    // Icons from https://humbleicons.com/;
    view! { cx,
        div(class="flex flex-col grow h-screen w-screen justify-center p-5 text-center") {
            div(class="flex flex-col items-center space-y-3") {
                h1(class="text-3xl") { strong{"SUTD Timetable to .ics convertor"} }
                p(class="text-sm text-warning") {
                    "Not sure what this is? "
                    a(href="readme", class="underline link-warning") { "Read me" }
                }
            }
            div(class="flex grow items-center justify-center") {
                div(class="flex flex-col justify-center space-y-3") {
                    h2(class="text-lg underline") { "SUTD Timetable: " }
                    button(class="btn flex-row space-x-3", on:click=|_| {
                        let document = web_sys::window().unwrap().document().unwrap();
                        document
                            .get_element_by_id("html_input")
                            .expect("#html_input element should exist in the page")
                            .dyn_ref::<web_sys::HtmlElement>().
                            expect("#html_input should be a html element")
                            .click();
                    }) {
                        h3 { "Import" }
                        svg(xmlns="http://www.w3.org/2000/svg", fill="none", stroke="currentColor", viewBox="0 0 24 24", class="w-6 h-6") {
                            path(xmlns="http://www.w3.org/2000/svg", stroke="currentColor", stroke-linecap="round", stroke-linejoin="round", stroke-width="2", d="M12 10v9m0-9l3 3m-3-3l-3 3m8.5 2c1.519 0 2.5-1.231 2.5-2.75a2.75 2.75 0 00-2.016-2.65A5 5 0 008.37 8.108a3.5 3.5 0 00-1.87 6.746") {}
                        }
                    }
                    input(type="file", accept=".html", class="hidden", id="html_input", on:change= |_| {
                        let document = web_sys::window().unwrap().document().unwrap();
                        let files = document
                            .get_element_by_id("html_input")
                            .expect("#html_input element should exist in the page")
                            .dyn_ref::<web_sys::HtmlInputElement>().
                            expect("#html_input should be a html input element")
                            .files()
                            .expect("#htmlInptut should have a file attribute");
                        let html_string = files
                            .item(0)
                            .expect("File should exist when on_change is trigger")
                            .text();
                        let closure = Closure::new(move |html_string: sycamore::rt::JsValue| {
                            let classes = utils::parse_classes(
                                &html_string
                                    .as_string()
                                    .expect("text function will return a string"),
                            );
                            let cal = utils::create_ics(&classes);
                            let cal = JsValue::from(vec![&cal.to_string()]
                                .iter()
                                .map(|x| JsValue::from_str(x))
                                .collect::<js_sys::Array>());
                            let cal = web_sys::Blob::new_with_str_sequence(&cal).expect("str to blob shouldn't fail");
                            let cal_url = web_sys::Url::create_object_url_with_blob(&cal).expect("url convertion shouldn't fail");
                            let download_btn = document
                                .create_element("a")
                                .unwrap();
                            let download_btn = download_btn
                                .dyn_ref::<web_sys::HtmlElement>()
                                .expect("#download_btn should be a html element");
                            download_btn.set_attribute("href", &cal_url);
                            download_btn.set_attribute("download", "timetable.ics");
                            download_btn.click();
                            web_sys::Url::revoke_object_url(&cal_url); // Not sure if this will cause problems, can be removed since users are unlikely to spam downloads
                        });
                        html_string.then(&closure);
                        unsafe {
                            closure.forget(); // Potential memory leak
                        }
                    }) {}
                }
            }
        }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Timetable Builder" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}
