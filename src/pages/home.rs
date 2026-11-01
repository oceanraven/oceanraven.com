use dioxus::html::completions::CompleteWithBraces::source;
use crate::components::{FeatureActivation, PageMeta, PageTitle};
use crate::content::{FEATURES, FEATURE_SOURCE, TOKENS};
use crate::AUTHOR;
use dioxus::prelude::*;


#[component]
pub fn Home() -> Element {
    rsx! {
        PageMeta {}
        div { class: "space-y-6",
            PageTitle { title: AUTHOR }
            FeatureActivation { tokens: TOKENS, features: FEATURES, source: &FEATURE_SOURCE }
        }
    }
}
