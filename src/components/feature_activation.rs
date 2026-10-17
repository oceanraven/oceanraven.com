use dioxus::prelude::*;

const MAX_HIGHLIGHT: f32 = 55.0;

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

fn acivation_style(activation: f32) -> String {
    if activation == 0.0 {
        return String::new();
    }
    let color_variable = if activation > 0.0 {
        "--color-primary"
    } else {
        "--color-secondary"
    };
    let mix_ratio = activation.abs().min(1.0) * MAX_HIGHLIGHT;
    format!("background-color: color-mix(in oklab, var({color_variable}) {mix_ratio:.0}%, transparent);")
}

#[component]
pub fn FeatureActivation(tokens: &'static [&'static str], features: &'static [Feature]) -> Element {
    let current_feature = &features[4];
    rsx! {
        p { class: "text-lg leading-relaxed",
            for (i, token) in tokens.iter().enumerate() {
                span {
                    key: "{i}",
                    style: acivation_style(current_feature.activation(i)),
                    title: format!("{:.2}", current_feature.activation(i)),
                    "{token}"
                }
            }

        }
    }
}
