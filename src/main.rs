mod components;
mod pages;
mod posts;

use dioxus::prelude::*;

use components::*;
use pages::*;
use posts::SYNTAX_CSS;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/research")]
    Research {},
    #[route("/projects")]
    Projects {},
    #[route("/cv")]
    Cv {},
    #[route("/blog")]
    BlogIndex {},
    #[route("/blog/:slug")]
    BlogPost { slug: String },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FONTS: Asset = asset!(
    "/assets/fonts",
    AssetOptions::folder().with_hash_suffix(false)
);
const KATEX: Asset = asset!(
    "/assets/katex",
    AssetOptions::folder().with_hash_suffix(false)
);

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link {
            rel: "preload",
            href: format!("{FONTS}/Inter-Variable.woff2"),
            r#as: "font",
            r#type: "font/woff2",
            crossorigin: "anonymous",
        }
        document::Link { rel: "stylesheet", href: format!("{KATEX}/katex.min.css") }
        document::Style { {SYNTAX_CSS} }
        Router::<Route> {}
    }
}
