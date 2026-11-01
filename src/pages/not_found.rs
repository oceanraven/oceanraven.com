use dioxus::prelude::*;
use crate::components::{PageMeta, PageTitle};

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let route: String = route.join("/");
    rsx! {
        PageMeta { title: "Not found" }
        Meta { name: "robots", content: "noindex" }
        div {
            PageTitle { title: "404 Not Found" }
            p { "This neuron is dead. No feature activates on /{route}." }
        }
    }
}
