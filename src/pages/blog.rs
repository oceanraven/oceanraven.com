use crate::pages::NotFound;
use crate::posts::{all_posts, find_post, render_markdown};
use crate::Route;
use dioxus::prelude::*;
use crate::components::TagList;

#[component]
pub fn BlogIndex() -> Element {
    rsx! {
        div {
            h1 { "Blog" }
            for post in all_posts().iter() {
                div { key: "{post.slug}",
                    Link {
                        to: Route::BlogPost {
                            slug: post.slug.clone(),
                        },
                        "{post.frontmatter.title}"
                    }
                    p { class: "font-mono", "{post.frontmatter.date}" }
                    TagList { tags: post.frontmatter.tags.clone() }
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
                    h1 { "{post.frontmatter.title}" }
                    div { class: "flex font-mono gap-2",
                        p { "{post.frontmatter.date}" }
                        TagList { tags: post.frontmatter.tags.clone() }
                    }
                    div { dangerous_inner_html: html_output }
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
