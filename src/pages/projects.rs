use dioxus::prelude::*;
use crate::components::PageTitle;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div {
            PageTitle { title: "Projects" }
            p { "This is the projects page." }
        }
    }
}