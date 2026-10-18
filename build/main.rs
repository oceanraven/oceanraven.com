mod markdown;
mod posts;

use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=posts");
    println!("cargo:rerun-if-changed=content");

    let renderer = markdown::Renderer::new();

    let mut code = String::new();
    code.push_str(&posts::generate(&renderer));
    code.push_str(&format!(
        "pub static SYNTAX_CSS: &str = {:?};\n",
        markdown::syntax_css()
    ));

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR should be set by Cargo for build scripts");
    fs::write(Path::new(&out_dir).join("posts_generated.rs"), code)
        .expect("generated code should be writable to OUT_DIR");
}