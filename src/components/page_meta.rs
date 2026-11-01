use dioxus::prelude::*;

use crate::{Route, AUTHOR, SITE_DESCRIPTION, SITE_URL};

const OG_IMAGE: Asset = asset!("/assets/og.png");

#[component]
pub fn PageMeta(
    /// Page name shown before the site name. None means the home page.
    title: Option<&'static str>,
    #[props(default = SITE_DESCRIPTION)] description: &'static str,
    /// Publication date for blog posts (YYYY-MM-DD).
    published: Option<&'static str>,
) -> Element {
    let route = use_route::<Route>();
    let url = format!("{SITE_URL}{route}");
    let image = format!("{SITE_URL}{OG_IMAGE}");

    let full_title = match title {
        Some(title) => format!("{title} · {AUTHOR}"),
        None => AUTHOR.to_string(),
    };
    let og_type = if published.is_some() { "article" } else { "website" };

    rsx! {
        document::Title { "{full_title}" }
        document::Meta { name: "description", content: description }

        document::Meta { property: "og:type", content: og_type }
        document::Meta { property: "og:site_name", content: AUTHOR }
        document::Meta { property: "og:title", content: full_title.clone() }
        document::Meta { property: "og:description", content: description }
        document::Meta { property: "og:url", content: url }
        document::Meta { property: "og:image", content: image }
        document::Meta { property: "og:image:width", content: "1200" }
        document::Meta { property: "og:image:height", content: "630" }
        document::Meta { property: "og:image:alt", content: "{AUTHOR}" }
        if let Some(date) = published {
            document::Meta { property: "article:published_time", content: date }
        }

        document::Meta { name: "twitter:card", content: "summary_large_image" }
    }
}