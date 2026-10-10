use dioxus::prelude::*;
use crate::components::PageTitle;

#[component]
pub fn Cv() -> Element {
    rsx! {
        div {
            PageTitle { title: "CV" }
            p { "This is the CV page." }
        }
    }
}