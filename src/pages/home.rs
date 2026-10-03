use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            h1 { "Seokyoung Yoon" }
            p { "Undergraduate student at Pohang University of Science and Technology." }
        }
    }
}
