use crate::components::ExternalLinks;
use crate::content::Publication;
use crate::AUTHOR;
use dioxus::prelude::*;

#[component]
pub fn PublicationItem(publication: &'static Publication) -> Element {
    rsx! {
        li {
            key: "{publication.year}-{publication.title}",
            class: "flex flex-col gap-1 py-4 sm:flex-row sm:items-baseline sm:gap-6",
            time {
                datetime: "{publication.year}",
                class: "shrink-0 font-mono text-sm text-neutral-600 dark:text-neutral-400",
                "{publication.year}"
            }
            div { class: "flex flex-col gap-1",
                p { class: "font-semibold", "{publication.title}" }

                p { class: "text-sm text-neutral-600 dark:text-neutral-400",
                    for (i, author) in publication.authors.iter().enumerate() {
                        span { key: "{i}",
                            if i > 0 {
                                ", "
                            }
                            if *author == AUTHOR {
                                strong { class: "font-semibold text-dark dark:text-light",
                                    "{author}"
                                }
                            } else {
                                "{author}"
                            }
                        }
                    }
                }

                p { class: "text-sm text-neutral-600 dark:text-neutral-400",
                    "{publication.venue} · "
                    span { class: "font-mono", "{publication.kind}" }
                }

                if !publication.links.is_empty() {
                    div { class: "flex gap-3 font-mono text-sm",
                        ExternalLinks { links: publication.links }
                    }
                }
            }
        }
    }
}
