use crate::components::META_LINK_CLASS;
use crate::content::{Feature, FeatureSource};
use dioxus::prelude::*;

struct TokenView {
    position: usize,
    text: &'static str,
    activation: f32,
    highlight_class: &'static str,
    highlight_style: String,
    hover_class: &'static str,
}

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
    let mut hovered = use_signal(|| None::<usize>);

    let current_feature = &features[index()];
    let hovered_position = hovered();

    let highlighted = tokens.iter().enumerate().map(|(i, token)| {
        let activation = current_feature.activation(i);
        let hover_class = if hovered_position == Some(i) {
            "underline underline-offset-4"
        } else {
            ""
        };
        let (highlight_class, highlight_style) = highlight(activation);
        TokenView {
            position: i,
            text: *token,
            activation,
            highlight_class,
            highlight_style,
            hover_class,
        }
    });

    let neuronpedia_url = format!(
        "https://www.neuronpedia.org/{}/{}",
        source.neuronpedia, current_feature.id
    );

    let readout = hovered_position.map(|position| {
        let activation = current_feature.activation(position);
        let value = if activation == 0.0 {
            "—".to_string()
        } else {
            format!("{activation:.2}")
        };
        format!("{:?} · {value}", tokens[position])
    });

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
            for TokenView {
        position,
        text,
        highlight_class,
        highlight_style,
        hover_class,
        activation: _,
    } in highlighted {
                span {
                    key: "{position}",
                    class: "transition-colors duration-300 cursor-default {highlight_class} decoration-secondary decoration-2 {hover_class}",
                    style: highlight_style,
                    onmouseenter: move |_| hovered.set(Some(position)),
                    onmouseleave: move |_| hovered.set(None),
                    onclick: move |_| {
                        let next = if hovered() == Some(position) { None } else { Some(position) };
                        hovered.set(next);
                    },
                    "{text}"
                }
            }
        }
        div { class: "mt-3 flex justify-between gap-4 font-mono text-xs text-neutral-600 dark:text-neutral-400",
            span { "{source.model} · layer {source.layer} · {source.sae} SAE" }
            if let Some(readout) = readout {
                span { "{readout}" }
            }
        }
    }
}
