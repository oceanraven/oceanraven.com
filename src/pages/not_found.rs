use dioxus::prelude::*;
use crate::components::PageTitle;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let route: String = route.join("/");
    rsx! {
        div {
            PageTitle { title: "404 Not Found" }
            p { "This neuron is dead. No feature activates on /{route}." }
        }
    }
}
