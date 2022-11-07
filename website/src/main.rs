mod error_pages;
mod templates;

use perseus::{Html, PerseusApp, PerseusRoot};
use sycamore::view;

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::readme::get_template)
        .error_pages(crate::error_pages::get_error_pages)
        .index_view(|cx| {
            view! { cx,
                head {
                    link(rel = "stylesheet", href = ".perseus/static/styles/tailwind.css")
                    link(rel = "icon", type = "image/png", href = ".perseus/static/favicon.png")
                }
                body {
                    PerseusRoot()
                }
            }
        })
}
