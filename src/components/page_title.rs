use dioxus::prelude::*;

#[component]
pub fn PageTitle(title: String) -> Element {
    rsx! {
        h1 {
            class: "text-3xl font-bold tracking-tight sm:text-4xl",
            "{title}"
        }
    }
}