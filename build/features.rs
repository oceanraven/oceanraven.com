use std::fs;
use serde::Deserialize;

const PATH: &str = "content/features.json";

#[derive(Deserialize)]
struct FeatureFile {
    source: SourceData,
    tokens: Vec<String>,
    features: Vec<FeatureData>
}

#[derive(Deserialize)]
struct SourceData {
    model:String,
    sae: String,
    layer: u32,
    neuronpedia: String,
}

#[derive(Deserialize)]
struct FeatureData {
    id: u32,
    label: String,
    activations: Vec<(usize, f32)>,
}
pub fn generate() -> String {
    let raw = fs::read_to_string(PATH).unwrap_or_else(|e| panic!("{PATH}: {e}"));
    let file: FeatureFile = serde_json::from_str(&raw).unwrap_or_else(|e| panic!("{PATH}: {e}"));
    for feature in &file.features {
        for &(position, value) in &feature.activations {
            if position >= file.tokens.len() {
                panic!(
                    "{PATH}: feature {} has activation at position {position}, but there are only {} tokens",
                    feature.id,
                    file.tokens.len(),
                );
            }

            if !(value > 0.0 && value <= 1.0) {
                panic!("{PATH}: feature {} has activation {value} at position {position}, expected (0, 1]", feature.id);
            }
        }
    }

    let mut code = String::new();
    code.push_str(&format!("pub static TOKENS: &[&str] = &{:?};\n", file.tokens));
    code.push_str(&format!(
        "pub static FEATURE_SOURCE: FeatureSource = FeatureSource {{ model: {:?}, sae: {:?}, layer: {}, neuronpedia: {:?} }};\n",
        file.source.model,
        file.source.sae,
        file.source.layer,
        file.source.neuronpedia,
    ));
    code.push_str("pub static FEATURES: &[Feature] = &[\n");
    for feature in &file.features {
        code.push_str(&format!(
            "    Feature {{ id: {}, label: {:?}, activations: &{:?} }},\n",
            feature.id,
            feature.label,
            feature.activations,
        ));
    }
    code.push_str("];\n");
    code
}