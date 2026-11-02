use crate::content::SparsityStep;
use dioxus::prelude::*;

const VIEW_BOX: &str = "-1.6 -1.6 3.2 3.2";
const LABEL_MIN_NORM: f32 = 0.1;
const LABEL_GAP: f32 = 0.18;
const INTERFERENCE_MIN: f32 = 0.05;
const DIMMED_OPACITY: f32 = 0.2;

struct ArrowView {
    index: usize,
    tip: (f32, f32),
    label: Option<(f32, f32)>,
    opacity: f32,
    stroke_class: &'static str,
    fill_class: &'static str,
}

fn to_svg((x, y): (f32, f32)) -> (f32, f32) {
    (x, -y)
}

fn dot((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> f32 {
    x1 * x2 + y1 * y2
}

fn emphasis(value: f32, min: f32, max: f32) -> f32 {
    if max - min < f32::EPSILON {
        return 1.0;
    }
    0.4 + 0.6 * (value - min) / (max - min)
}

fn interference_label(vectors: &[(f32, f32)], index: usize) -> String {
    let source = vectors[index];
    let norm = source.0.hypot(source.1);
    let others: Vec<String> = vectors
        .iter()
        .enumerate()
        .filter(|&(other, _)| other != index)
        .map(|(other, &vector)| (other, dot(source, vector)))
        .filter(|&(_, value)| value > INTERFERENCE_MIN)
        .map(|(other, value)| format!("{other} ({value:.2})"))
        .collect();

    if others.is_empty() {
        format!("feature {index} · ‖W‖ {norm:.2} · no interference")
    } else {
        format!(
            "feature {index} · ‖W‖ {norm:.2} · interferes with {}",
            others.join(", ")
        )
    }
}

#[component]
pub fn SuperpositionPlot(steps: &'static [SparsityStep], importance: &'static [f32]) -> Element {
    let mut selected = use_signal(|| steps.len() - 1);
    let mut hovered = use_signal(|| None::<usize>);
    let step = &steps[selected()];
    let hovered_index = hovered();

    let (min_importance, max_importance) = importance
        .iter()
        .fold((f32::MAX, f32::MIN), |(low, high), &value| (low.min(value), high.max(value)));

    let arrows = step.vectors.iter().enumerate().map(|(index, &(x, y))| {
        let norm = x.hypot(y);
        let label = (norm >= LABEL_MIN_NORM).then(|| {
            let scale = (norm + LABEL_GAP) / norm;
            to_svg((x * scale, y * scale))
        });

        let (opacity, stroke_class, fill_class) = match hovered_index {
            None => (
                emphasis(importance[index], min_importance, max_importance),
                "stroke-primary",
                "fill-primary",
            ),
            Some(source) if source == index => (1.0, "stroke-primary", "fill-primary"),
            Some(source) => {
                let value = dot(step.vectors[source], (x, y));
                if value > INTERFERENCE_MIN {
                    (0.4 + 0.6 * value.min(1.0), "stroke-secondary", "fill-secondary")
                } else {
                    (DIMMED_OPACITY, "stroke-primary", "fill-primary")
                }
            }
        };

        ArrowView {
            index,
            tip: to_svg((x, y)),
            label,
            opacity,
            stroke_class,
            fill_class,
        }
    });

    let readout = hovered_index.map(|index| interference_label(step.vectors, index));

    rsx! {
        div {
            svg {
                view_box: VIEW_BOX,
                class: "mx-auto w-full max-w-xs",
                role: "img",
                onclick: move |_| hovered.set(None),
                circle {
                    cx: "0",
                    cy: "0",
                    r: "1",
                    fill: "none",
                    stroke_width: "0.01",
                    stroke_dasharray: "0.04 0.04",
                    class: "stroke-neutral-300 dark:stroke-neutral-700",
                }
                for ArrowView { index, tip: (tx, ty), label, opacity, stroke_class, fill_class } in arrows {
                    g { key: "{index}",
                        line {
                            x1: "0",
                            y1: "0",
                            x2: "{tx}",
                            y2: "{ty}",
                            stroke_width: "0.03",
                            stroke_linecap: "round",
                            stroke_opacity: "{opacity:.2}",
                            class: stroke_class,
                        }
                        circle {
                            cx: "{tx}",
                            cy: "{ty}",
                            r: "0.045",
                            fill_opacity: "{opacity:.2}",
                            class: fill_class,
                        }
                        if let Some((lx, ly)) = label {
                            text {
                                x: "{lx}",
                                y: "{ly}",
                                font_size: "0.14",
                                text_anchor: "middle",
                                dominant_baseline: "middle",
                                class: "fill-neutral-600 font-mono dark:fill-neutral-400",
                                "{index}"
                            }
                            circle {
                                cx: "{tx}",
                                cy: "{ty}",
                                r: "0.16",
                                fill: "transparent",
                                class: "cursor-pointer",
                                onpointerenter: move |event| {
                                    if event.pointer_type() == "mouse" {
                                        hovered.set(Some(index));
                                    }
                                },
                                onpointerleave: move |event| {
                                    if event.pointer_type() == "mouse" {
                                        hovered.set(None);
                                    }
                                },
                                onclick: move |event| {
                                    event.stop_propagation();
                                    hovered.set(Some(index));
                                },
                            }
                        }
                    }
                }
            }

            div { class: "mt-4 flex items-center gap-4 font-mono text-xs text-neutral-600 dark:text-neutral-400",
                label { r#for: "sparsity", class: "shrink-0 tabular-nums",
                    "sparsity {step.sparsity:.2}"
                }
                input {
                    id: "sparsity",
                    r#type: "range",
                    min: "0",
                    max: "{steps.len() - 1}",
                    step: "1",
                    value: "{selected}",
                    class: "w-full cursor-pointer accent-primary",
                    oninput: move |event| {
                        if let Ok(value) = event.value().parse::<usize>() {
                            selected.set(value);
                            hovered.set(None);
                        }
                    },
                }
            }

            p {
                class: "mt-2 font-mono text-xs text-neutral-600 dark:text-neutral-400",
                aria_live: "polite",
                if let Some(text) = readout {
                    "{text}"
                } else {
                    "Hover or tap an arrow to see which features it interferes with."
                }
            }
        }
    }
}