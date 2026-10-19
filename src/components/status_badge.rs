use dioxus::prelude::*;
use crate::content::Status;

#[component]
pub fn StatusBadge(status: Status) -> Element{
    rsx! {
        span { class: "inline-flex items-center gap-1.5 font-mono text-xs text-neutral-600 dark:text-neutral-400",
            span {
                aria_hidden: "true",
                class: format!("inline-block size-2 rounded-full {}", status.dot_class()),
            }
            "{status.label()}"
        }
    }
}