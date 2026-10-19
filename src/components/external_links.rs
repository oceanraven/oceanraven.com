use dioxus::prelude::*;

pub const META_LINK_CLASS: &str = "text-neutral-600 transition-colors hover:text-primary-deep \
    dark:text-neutral-400 dark:hover:text-primary";

#[component]
pub fn ExternalLinks(links: &'static [(&'static str, &'static str)]) -> Element {
    rsx! {
        for (label, url) in links {
            a {
                key: "{label}",
                href: "{url}",
                target: "_blank",
                rel: "noopener noreferrer",
                class: META_LINK_CLASS,
                "[{label}]"
            }
        }
    }
}
