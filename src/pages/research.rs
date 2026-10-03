use dioxus::prelude::*;

#[component]
pub fn Research() -> Element {
    rsx! {
        div {
            h1 { "Research" }
            p { "This is the research page." }
        }
    }
}