use serde::Deserialize;
use std::fs;

const PATH: &str = "content/superposition.json";

#[derive(Deserialize)]
struct SuperpositionFile {
    importance: Vec<f32>,
    steps: Vec<StepData>,
}

#[derive(Deserialize)]
struct StepData {
    sparsity: f32,
    vectors: Vec<(f32, f32)>,
}

fn validate(file: &SuperpositionFile) {
    if file.importance.is_empty() {
        panic!("{PATH}: importance should not be empty");
    }
    if file.steps.is_empty() {
        panic!("{PATH}: steps should not be empty");
    }

    for step in &file.steps {
        if step.vectors.len() != file.importance.len() {
            panic!(
                "{PATH}: sparsity {:?} has {} vectors, expected {}",
                step.sparsity,
                step.vectors.len(),
                file.importance.len(),
            );
        }
        if !(0.0..1.0).contains(&step.sparsity) {
            panic!("{PATH}: sparsity {:?} should be in [0, 1)", step.sparsity);
        }
    }

    for pair in file.steps.windows(2) {
        if pair[0].sparsity >= pair[1].sparsity {
            panic!(
                "{PATH}: sparsity {:?} is followed by {:?}, expected increasing values",
                pair[0].sparsity, pair[1].sparsity,
            );
        }
    }
}

pub fn generate() -> String {
    let raw = fs::read_to_string(PATH).unwrap_or_else(|e| panic!("{PATH}: {e}"));
    let file: SuperpositionFile =
        serde_json::from_str(&raw).unwrap_or_else(|e| panic!("{PATH}: {e}"));

    validate(&file);

    let mut code = format!(
        "pub static FEATURE_IMPORTANCE: &[f32] = &{:?};\n",
        file.importance
    );

    code.push_str("pub static SUPERPOSITION: &[SparsityStep] = &[\n");
    for step in &file.steps {
        code.push_str(&format!(
            "    SparsityStep {{ sparsity: {:?}, vectors: &{:?} }},\n",
            step.sparsity, step.vectors,
        ));
    }
    code.push_str("];\n");

    code
}
