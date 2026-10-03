use dioxus::prelude::*;

#[component]
pub fn Cv() -> Element {
    rsx! {
        div {
            h1 { "CV" }
            p { "This is the CV page." }
        }
    }
}