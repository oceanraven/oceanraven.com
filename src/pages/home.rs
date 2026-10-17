use dioxus::prelude::*;
use crate::components::{Feature, FeatureActivation, PageTitle};


#[rustfmt::skip]
const TOKENS: &[&str] = &[
    "I", "’m", " an", " undergraduate", " at", " POST", "ECH", " working", " on", " mechan", "istic", // 0
    " interpret", "ability", ".",                                                                     // 11
    " I", " try", " to", " reverse", "-", "engine", "er", " how", " neural", " networks",             // 14
    " represent", " concepts", " and", " carry", " out", " comput", "ations", ",",                    // 24
    " one", " circuit", " at", " a", " time", ".",                                                    // 32
    " When", " I", "’m", " not", " staring", " at", " act", "ivations", ",",                          // 38
    " I", " build", " things", " in", " Rust", ",", " including", " this", " site", ".",              // 47
];

const FEATURES: &[Feature] = &[
    Feature {
        id: 4127,
        label: "interpretability & ML research concepts",
        activations: &[
            (9, 0.55), (10, 0.80), (11, 0.70), (12, 0.95),
            (17, 0.35), (19, 0.45), (20, 0.40),
            (22, 0.75), (23, 0.85), (24, 0.50), (25, 0.60),
            (29, 0.45), (30, 0.55), (33, 0.90),
            (44, 0.70), (45, 0.92),
        ],
    },
    Feature {
        id: 1893,
        label: "first-person self-reference",
        activations: &[
            (0, 0.95), (1, 0.40),
            (14, 0.90), (15, 0.30),
            (39, 0.88), (40, 0.35),
            (47, 0.93), (48, 0.25),
        ],
    },
    Feature {
        id: 702,
        label: "proper nouns: institutions & languages",
        activations: &[
            (3, 0.15), (5, 0.85), (6, 0.98), (51, 0.90),
        ],
    },
    Feature {
        id: 9311,
        label: "sentence & clause boundaries",
        activations: &[
            (13, 0.90), (31, 0.55), (37, 0.92), (38, 0.30),
            (46, 0.50), (52, 0.48), (56, 0.95),
        ],
    },
    Feature {
        id: 2268,
        label: "building software (+) vs. studying models (−)",
        activations: &[
            (19, 0.40), (20, 0.35),
            (48, 0.85), (49, 0.55), (51, 0.75), (54, 0.30), (55, 0.80),
            (9, -0.45), (10, -0.50), (11, -0.55), (12, -0.60),
            (22, -0.40), (23, -0.45), (44, -0.50), (45, -0.55),
        ],
    },
];

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            PageTitle { title: "Seokyoung Yoon" }
            FeatureActivation { tokens: TOKENS, features: FEATURES }
        }
    }
}
