use crate::components::{PageTitle, TagList};
use crate::pages::NotFound;
use crate::posts::{all_posts, find_post, render_markdown};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogIndex() -> Element {
    rsx! {
        div {
            PageTitle { title: "Blog" }
            ul { class: "divide-y divide-neutral-200 dark:divide-neutral-800",
                for post in all_posts() {
                    li {
                        key: "{post.slug}",
                        class: "flex flex-col gap-1 py-4 sm:flex-row sm:items-baseline sm:gap-6",
                        time {
                            datetime: "{post.frontmatter.date}",
                            class: "shrink-0 font-mono text-sm text-neutral-600 dark:text-neutral-400",
                            "{post.frontmatter.date}"
                        }
                        div { class: "flex flex-col gap-1",
                            Link {
                                class: "text-lg font-semibold transition-colors hover:text-primary-deep dark:hover:text-primary",
                                to: Route::BlogPost { slug: post.slug.clone() },
                                "{post.frontmatter.title}"
                            }
                            TagList { tags: post.frontmatter.tags.clone() }
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
            let html_output = render_markdown(&post.body);

            rsx! {
                div {
                    PageTitle { title: &post.frontmatter.title }
                    div { class: "flex font-mono gap-2 my-4",
                        time { class: "text-neutral-600 dark:text-neutral-400", "{post.frontmatter.date}" }
                        TagList { tags: post.frontmatter.tags.clone() }
                    }
                    hr { class: "my-8 border-neutral-200 dark:border-neutral-800" }
                    div {
                        class: "prose prose-lg prose-neutral dark:prose-invert max-w-none \
                        prose-a:text-primary-deep hover:prose-a:text-primary dark:prose-a:text-primary dark:hover:prose-a:text-primary-deep \
                        prose-code:before:content-none prose-code:after:content-none prose-code:font-normal \
                        prose-code:px-1 prose-code:py-0.5 prose-code:rounded-sm \
                        prose-h2:mt-12 prose-h2:mb-4 prose-h2:text-2xl prose-h2:font-semibold prose-h2:tracking-tight \
                        prose-h3:mt-8 prose-h3:mb-3 prose-h3:text-xl prose-h3:font-semibold \
                        prose-h4:mt-6 prose-h4:mb-2 prose-h4:text-lg prose-h4:font-semibold",
                        dangerous_inner_html: html_output,
                    }
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
