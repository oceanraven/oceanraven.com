use crate::components::{PageTitle, ProjectItem};
use crate::content::PROJECTS;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "space-y-8",
            div { class: "space-y-3",

                PageTitle { title: "Projects" }
                p { class: "text-neutral-600 dark:text-neutral-400",
                    "Things I've built, mostly in Rust and Python."
                }
            }
            if PROJECTS.is_empty() {
                p { class: "text-neutral-600 dark:text-neutral-400", "Nothing here yet." }
            } else {
                ul { class: "divide-y divide-neutral-200 dark:divide-neutral-800",
                    for project in PROJECTS {
                        ProjectItem { key: "{project.name}", project }
                    }
                }
            }
        }
    }
}
