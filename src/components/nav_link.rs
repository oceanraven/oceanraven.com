use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn NavLink(to: Route, children: Element) -> Element {
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
            children,
        }
    }
}