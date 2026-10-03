use dioxus::prelude::*;

#[component]
pub fn BlogIndex() -> Element {
    rsx! {
        div {
            h1 { "Blog Home" }
            p { "This is the blog home page." }
        }
    }
}

#[component]
pub fn BlogPost(slug: String) -> Element {
    rsx! {
        div {
            h1 { "Blog" }
            p { "This is the blog page. {slug}" }
        }
    }
}

