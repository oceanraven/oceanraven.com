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

#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    let mut routes: Vec<String> = Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect();

    routes.extend(crate::posts::all_posts().iter().map(|post| {
        Route::BlogPost {
            slug: post.slug.to_string(),
        }
        .to_string()
    }));

    Ok(routes)
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
    LaunchBuilder::new()
        .with_cfg(server_only! {
            dioxus::server::ServeConfig::builder()
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        .static_dir(
                            std::env::current_exe()
                                .expect("server executable path should be available")
                                .parent()
                                .expect("server executable should be inside a directory")
                                .join("public"),
                        )
                        .clear_cache(false),
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Meta {charset: "utf-8"}
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
