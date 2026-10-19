use crate::components::{ExternalLinks, StatusBadge, TagList, META_LINK_CLASS};
use crate::content::Project;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn ProjectItem(project: &'static Project) -> Element {
    let has_footer =
        !project.tags.is_empty() || !project.links.is_empty() || project.post.is_some();

    rsx! {
        li { class: "flex flex-col gap-2 py-4",
            div { class: "flex items-baseline justify-between gap-4",
                h2 { class: "font-semibold", "{project.name}" }
                div { class: "flex shrink-0 items-center gap-2 font-mono text-xs text-neutral-600 dark:text-neutral-400",
                    span { "{project.year}" }
                    span { aria_hidden: "true", "·" }
                    StatusBadge { status: project.status }
                }
            }

            p { class: "text-neutral-600 dark:text-neutral-400", "{project.summary}" }

            if has_footer {
                div { class: "flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1",
                    TagList { tags: project.tags }
                    div { class: "flex gap-3 font-mono text-sm",
                        ExternalLinks { links: project.links }
                        if let Some(slug) = project.post {
                            Link {
                                to: Route::BlogPost {
                                    slug: slug.to_string(),
                                },
                                class: META_LINK_CLASS,
                                "[post]"
                            }
                        }
                    }
                }
            }
        }
    }
}