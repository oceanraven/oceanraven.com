use dioxus::prelude::*;
use crate::components::PageTitle;

#[component]
pub fn Research() -> Element {
    rsx! {
        div {
            PageTitle { title: "Research" }
            p { "This is the research page." }
        }
    }
}