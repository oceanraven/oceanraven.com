use crate::components::META_LINK_CLASS;
use crate::content::{Feature, FeatureSource};
use dioxus::prelude::*;

fn highlight(activation: f32) -> (&'static str, String) {
    if activation == 0.0 {
        return ("", String::new());
    }
    ("highlight", format!("--activation: {:.2}", activation))
}

#[component]
pub fn FeatureActivation(
    tokens: &'static [&'static str],
    features: &'static [Feature],
    source: &'static FeatureSource,
) -> Element {
    let mut index = use_signal(|| 0);
    let current_feature = &features[index()];

    let highlighted = tokens.iter().enumerate().map(|(i, token)| {
        let activation = current_feature.activation(i);
        let (highlight_class, highlight_style) = highlight(activation);
        (
            i,
            *token,
            highlight_class,
            highlight_style,
            format!("{activation:.2}"),
        )
    });

    let neuronpedia_url = format!(
        "https://www.neuronpedia.org/{}/{}",
        source.neuronpedia, current_feature.id
    );

    rsx! {
        div { class: "mb-4 flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1 font-mono text-sm text-neutral-600 dark:text-neutral-400",
            span { aria_live: "polite",
                a {
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "{META_LINK_CLASS} underline decoration-dotted underline-offset-4",
                    href: neuronpedia_url,
                    "feature #{current_feature.id}"
                }
                " · {current_feature.label}"
            }
            button {
                class: "shrink-0 cursor-pointer transition-colors hover:text-dark dark:hover:text-light",
                onclick: move |_| index.set((index() + 1) % features.len()),
                "next feature ({index() + 1}/{features.len()}) →"
            }
        }
        p { class: "text-lg leading-relaxed",
            for (i, token, highlight_class, highlight_style, tooltip) in highlighted {
                span {
                    key: "{i}",
                    class: "transition-colors duration-300 {highlight_class}",
                    style: highlight_style,
                    title: tooltip,
                    "{token}"
                }
            }
        }
        p {
            class: "mt-3 font-mono text-xs text-neutral-600 dark:text-neutral-400",
            "{source.model} · layer {source.layer} · {source.sae} SAE"
        }
    }
}
