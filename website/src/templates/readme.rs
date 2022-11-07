use perseus::Template;
use sycamore::prelude::{view, Html, Scope, SsrNode, View};

const MARKDOWN: &str = include_str!("../../readme.md");

#[perseus::template_rx]
pub fn readme_page<G: Html>(cx: Scope) -> View<G> {
    // Icons from https://humbleicons.com/
    let parsed = mdsycx::parse::<()>(MARKDOWN).unwrap();
    view! { cx,
        div(class="flex flex-col items-center") {
            div(class="relative p-5 w-full flex md:fixed md:left-0 md:w-auto") {
                a(class="btn md:bg-transparent md:border-0 flex-grow", href="") {
                    svg(class="w-10 h-10", xmlns="http://www.w3.org/2000/svg", fill="none", stroke="currentColor", viewBox="0 0 24 24") {
                        path(xmlns="http://www.w3.org/2000/svg", stroke="currentColor", stroke-linecap="round", stroke-linejoin="round", stroke-width="2", d="M11 18h3.75a5.25 5.25 0 100-10.5H5M7.5 4L4 7.5 7.5 11") {}
                    }
                }
            }
            article(class="flex flex-col max-w-xl justify-center p-5 px-10 prose prose-headings:place-self-center prose-headings:text-center") {
                mdsycx::MDSycX(body=parsed.body)
            }
        }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "readme.md" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("readme").template(readme_page).head(head)
}
