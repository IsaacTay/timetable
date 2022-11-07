use std::error;

use perseus::{ErrorPages, Html};
use sycamore::view;

pub fn get_error_pages<G: Html>() -> ErrorPages<G> {
    let mut error_pages = ErrorPages::new(
        |cx, url, status, err, _| {
            view! { cx,
                p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
            }
        },
        |cx, _, _, _, _| {
            view! { cx,
                title { "Error" }
            }
        },
    );

    error_pages
}
