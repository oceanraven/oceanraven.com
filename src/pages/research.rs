use crate::components::{PageTitle, Prose, PublicationItem};
use crate::content::{PUBLICATIONS, RESEARCH_HTML};
use dioxus::prelude::*;

#[component]
pub fn Research() -> Element {
    rsx! {
        div { class: "space-y-8",
            PageTitle { title: "Research" }
            Prose { html: RESEARCH_HTML }

            section {
                h2 { class: "mb-4 text-2xl font-semibold tracking-tight", "Publications & Talks" }

                if PUBLICATIONS.is_empty() {
                    p { class: "text-neutral-600 dark:text-neutral-400", "Nothing here yet." }
                } else {
                    ul { class: "divide-y divide-neutral-200 dark:divide-neutral-800",
                        for publication in PUBLICATIONS {
                            PublicationItem {
                                key: "{publication.year}-{publication.title}",
                                publication,
                            }
                        }
                    }
                }
            }
        }
    }
}
