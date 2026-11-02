use crate::components::META_LINK_CLASS;
use crate::content::{Feature, FeatureSource};
use dioxus::prelude::*;

const HOVER_CLASS: &str = "underline decoration-secondary decoration-2 underline-offset-4";

struct TokenView {
    position: usize,
    text: &'static str,
    highlight_class: &'static str,
    highlight_style: String,
    hover_class: &'static str,
}

fn highlight(activation: f32) -> (&'static str, String) {
    if activation == 0.0 {
        return ("", String::new());
    }
    ("highlight", format!("--activation: {activation:.2}"))
}

#[component]
pub fn FeatureActivation(
    tokens: &'static [&'static str],
    features: &'static [Feature],
    source: &'static FeatureSource,
) -> Element {
    let mut index = use_signal(|| 0);
    let mut hovered = use_signal(|| None::<usize>);

    let current_feature = &features[index()];
    let hovered_position = hovered();

    let highlighted = tokens.iter().enumerate().map(|(position, token)| {
        let (highlight_class, highlight_style) = highlight(current_feature.activation(position));
        TokenView {
            position,
            text: *token,
            highlight_class,
            highlight_style,
            hover_class: if hovered_position == Some(position) {
                HOVER_CLASS
            } else {
                ""
            },
        }
    });

    let readout = hovered_position.map(|position| {
        let activation = current_feature.activation(position);
        let value = if activation == 0.0 {
            "—".to_string()
        } else {
            format!("{activation:.2}")
        };
        format!("{:?} · {value}", tokens[position])
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
        p {
            class: "text-lg leading-relaxed",
            onclick: move |_| hovered.set(None),
            for TokenView { position, text, highlight_class, highlight_style, hover_class } in highlighted {
                span {
                    key: "{position}",
                    class: "cursor-default transition-[background-color] duration-300 {highlight_class} {hover_class}",
                    style: highlight_style,
                    onpointerenter: move |event| {
                        if event.pointer_type() == "mouse" {
                            hovered.set(Some(position));
                        }
                    },
                    onpointerleave: move |event| {
                        if event.pointer_type() == "mouse" {
                            hovered.set(None);
                        }
                    },
                    onclick: move |event| {
                        event.stop_propagation();
                        hovered.set(Some(position));
                    },
                    "{text}"
                }
            }
        }
        div { class: "mt-3 flex flex-col gap-2 font-mono text-xs text-neutral-600 md:flex-row md:justify-between md:gap-4 dark:text-neutral-400",
            span { "{source.model} · layer {source.layer} · {source.sae} SAE" }
            if let Some(readout) = readout {
                span { "{readout}" }
            }
        }
    }
}
