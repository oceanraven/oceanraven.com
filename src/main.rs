use dioxus::prelude::*;

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

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "preload", href: format!("{FONTS}/Inter-Variable.woff2"), as: "font", type: "font/woff2", crossorigin: "anonymous" }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        div {
            h1 {
                "Seokyoung Yoon"
            }
            p {
                "Undergraduate student at Pohang University of Science and Technology."
            }
        }

    }
}

#[component]
fn Research() -> Element {
    rsx! {
        div {
            h1 { "Research" }
            p { "This is the research page." }
        }
    }
}

#[component]
fn Projects() -> Element {
    rsx! {
        div {
            h1 { "Projects" }
            p { "This is the projects page." }
        }
    }
}

#[component]
fn Cv() -> Element {
    rsx! {
        div {
            h1 { "CV" }
            p { "This is the CV page." }
        }
    }
}

#[component]
fn BlogIndex() -> Element {
    rsx! {
        div {
            h1 { "Blog Home" }
            p { "This is the blog home page." }
        }
    }
}

#[component]
fn BlogPost(slug: String) -> Element {
    rsx! {
        div {
            h1 { "Blog" }
            p { "This is the blog page. {slug}" }
        }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    let route: String = route.join("/");
    rsx! {
        div {
            h1 { "404 Not Found" }
            p { "This neuron is dead. No feature activates on /{route}." }
        }
    }
}

#[component]
fn NavLink(to: Route, children: Element) -> Element {
    let current_route = use_route::<Route>();
    let is_active = match &to {
        Route::BlogIndex { .. } => {
            matches!(
                current_route,
                Route::BlogIndex { .. } | Route::BlogPost { .. }
            )
        }
        _ => current_route == to,
    };
    let classes = if is_active {
        "text-primary-deep dark:text-primary"
    } else {
        "text-neutral-600 dark:text-neutral-400 hover:text-dark dark:hover:text-light"
    };
    rsx! {
        Link {
            to,
            class: format!("transition-colors {classes}"),
            children
        }
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        div {
            class: "mx-auto flex min-h-screen w-full max-w-3xl flex-col px-6",
            header {
                nav {
                    class: "flex flex-wrap items-center justify-between gap-y-2 py-6",
                    Link { class: "font-semibold", to: Route::Home {}, "Seokyoung Yoon" }
                    div {
                        class: "flex gap-4 font-mono text-sm",
                        NavLink { to: Route::Research {}, "Research" }
                        NavLink { to: Route::Projects {}, "Projects" }
                        NavLink { to: Route::Cv {}, "CV" }
                        NavLink { to: Route::BlogIndex {}, "Blog" }
                    }
                }
            }
            main {
                class: "flex-1 py-8",
                Outlet::<Route> {}
            }
            footer {
                class: "py-6 font-mono text-sm text-neutral-600 dark:text-neutral-400",
                p { "© 2026 Seokyoung Yoon." }
            }
        }
    }
}
