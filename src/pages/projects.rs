use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div {
            h1 { "Projects" }
            p { "This is the projects page." }
        }
    }
}