use dioxus::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let route: String = route.join("/");
    rsx! {
        div {
            h1 { "404 Not Found" }
            p { "This neuron is dead. No feature activates on /{route}." }
        }
    }
}
