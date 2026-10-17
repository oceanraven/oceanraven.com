use dioxus::prelude::*;

#[derive(PartialEq)]
pub struct Feature {
    pub id: u32,
    pub label: &'static str,
    pub activations: &'static [(usize, f32)],
}

impl Feature {
    fn activation(&self, index: usize) -> f32 {
        self.activations
            .iter()
            .find(|(i, _)| *i == index)
            .map(|(_, activation)| *activation)
            .unwrap_or(0.0)
    }
}

fn highlight(activation: f32) -> (&'static str, String) {
    if activation == 0.0 {
        return ("", String::new());
    }
    let class = if activation > 0.0 {
        "highlight-positive"
    } else {
        "highlight-negative"
    };
    let style = format!("--activation: {:.2}", activation.abs().min(1.0));
    (class, style)
}

#[component]
pub fn FeatureActivation(tokens: &'static [&'static str], features: &'static [Feature]) -> Element {
    let mut index = use_signal(|| 0);
    let current_feature = &features[index()];

    let highlighted = tokens.iter().enumerate().map(|(i, token)| {
        let activation = current_feature.activation(i);
        let (highlight_class, highlight_style) = highlight(activation);
        (i, *token, highlight_class, highlight_style, format!("{activation:.2}"))
    });

    rsx! {
        div { class: "mb-4 flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1 font-mono text-sm text-neutral-600 dark:text-neutral-400",
            span { aria_live: "polite", "feature #{current_feature.id} · {current_feature.label}" }
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
    }
}