use dioxus::prelude::*;
use crate::{Route, AUTHOR};
use super::NavLink;


#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "mx-auto flex min-h-screen w-full max-w-3xl flex-col px-6",
            header {
                nav { class: "flex flex-wrap items-center justify-between gap-y-2 py-6",
                    Link { class: "font-semibold", to: Route::Home {}, "{AUTHOR}" }
                    div { class: "flex gap-4 font-mono text-sm",
                        NavLink { to: Route::Research {}, "Research" }
                        NavLink { to: Route::Projects {}, "Projects" }
                        NavLink { to: Route::Cv {}, "CV" }
                        NavLink { to: Route::BlogIndex {}, "Blog" }
                    }
                }
            }
            main { class: "flex-1 py-8", Outlet::<Route> {} }
            footer { class: "py-6 font-mono text-sm text-neutral-600 dark:text-neutral-400",
                p { "© 2026 {AUTHOR}." }
            }
        }
    }
}
