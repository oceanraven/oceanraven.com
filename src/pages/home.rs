use dioxus::prelude::*;
use crate::components::PageTitle;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            PageTitle { title: "Seokyoung Yoon" }
            p { "Undergraduate student at Pohang University of Science and Technology." }
        }
    }
}
