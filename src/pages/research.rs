use crate::components::{PageMeta, PageTitle, Prose, PublicationItem, SuperpositionPlot};
use crate::content::{FEATURE_IMPORTANCE, PUBLICATIONS, RESEARCH_HTML, SUPERPOSITION};
use dioxus::prelude::*;

#[component]
pub fn Research() -> Element {
    rsx! {
            PageMeta {
                title: "Research",
                description: "Research interests, publications, and talks on language model interpretability.",
            }
            div { class: "space-y-8",
                PageTitle { title: "Research" }
                Prose { html: RESEARCH_HTML }

                section {
        h2 { class: "mb-4 text-2xl font-semibold tracking-tight", "A toy model of superposition" }
        p { class: "mb-6 text-neutral-600 dark:text-neutral-400",
            "Five features squeezed into two dimensions. Drag the slider to make features rarer: "
            "the model starts by keeping only the two most important ones, then packs them into "
            "opposite pairs, and finally into a pentagon. "
            a {
                href: "https://transformer-circuits.pub/2022/toy_model/index.html",
                target: "_blank",
                rel: "noopener noreferrer",
                class: "underline underline-offset-4 decoration-primary hover:text-primary-deep dark:hover:text-primary",
                "Elhage et al., 2022"
            }
            "."
        }
        SuperpositionPlot { steps: SUPERPOSITION, importance: FEATURE_IMPORTANCE }
    }

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
