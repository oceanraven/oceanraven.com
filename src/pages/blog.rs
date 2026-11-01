use crate::components::{PageMeta, PageTitle, Prose, TagList};
use crate::pages::NotFound;
use crate::posts::{all_posts, find_post};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogIndex() -> Element {
    let posts = all_posts();

    rsx! {
        PageMeta {
            title: "Blog",
            description: "Notes on interpretability, Rust, and things I'm learning.",
        }
        div { class: "space-y-8",
            PageTitle { title: "Blog" }

            if posts.is_empty() {
                p { class: "text-neutral-600 dark:text-neutral-400",
                    "Nothing here yet. The first post is on its way."
                }
            } else {
                ul { class: "divide-y divide-neutral-200 dark:divide-neutral-800",
                    for post in posts {
                        li {
                            key: "{post.slug}",
                            class: "flex flex-col gap-1 py-4 sm:flex-row sm:items-baseline sm:gap-6",
                            time {
                                datetime: "{post.date}",
                                class: "shrink-0 font-mono text-sm text-neutral-600 dark:text-neutral-400",
                                "{post.date}"
                            }
                            div { class: "flex flex-col gap-1",
                                Link {
                                    class: "text-lg font-semibold transition-colors hover:text-primary-deep dark:hover:text-primary",
                                    to: Route::BlogPost {
                                        slug: post.slug.to_string(),
                                    },
                                    "{post.title}"
                                }
                                TagList { tags: post.tags }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BlogPost(slug: String) -> Element {
    let post = find_post(&slug);

    match post {
        Some(post) => {
            rsx! {
                PageMeta {
                    title: post.title,
                    description: post.description.unwrap_or(crate::SITE_DESCRIPTION),
                    published: post.date,
                }
                div {
                    PageTitle { title: post.title }
                    div { class: "my-4 flex gap-2 font-mono",
                        time {
                            datetime: "{post.date}",
                            class: "text-neutral-600 dark:text-neutral-400",
                            "{post.date}"
                        }
                        TagList { tags: post.tags }
                    }
                    hr { class: "my-8 border-neutral-200 dark:border-neutral-800" }
                    Prose { html: post.html }
                }
            }
        }

        None => {
            rsx! {
                NotFound { route: vec!["blog".to_string(), slug] }
            }
        }
    }
}