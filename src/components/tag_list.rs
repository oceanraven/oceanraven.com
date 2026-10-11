use dioxus::prelude::*;

#[component]
pub fn TagList(tags: &'static [&'static str]) -> Element {
    rsx! {
        div { class: "flex gap-2 text-secondary-deep dark:text-secondary font-mono",
            for tag in tags {
                span { key: "{tag}", "#{tag}" }
            }
        }
    }
}
