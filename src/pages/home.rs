use crate::components::{FeatureActivation, PageMeta, PageTitle};
use crate::content::{FEATURES, TOKENS};
use crate::AUTHOR;
use dioxus::prelude::*;


#[component]
pub fn Home() -> Element {
    rsx! {
        PageMeta {}
        div { class: "space-y-6",
            PageTitle { title: AUTHOR }
            FeatureActivation { tokens: TOKENS, features: FEATURES }
        }
    }
}
