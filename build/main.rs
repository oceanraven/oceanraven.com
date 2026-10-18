mod content;
mod markdown;
mod posts;

use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=posts");
    println!("cargo:rerun-if-changed=content");

    let renderer = markdown::Renderer::new();

    let mut posts_code = posts::generate(&renderer);
    posts_code.push_str(&format!(
        "pub static SYNTAX_CSS: &str = {:?};\n",
        markdown::syntax_css()
    ));

    let content_code = content::generate(&renderer);

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR should be set by Cargo for build scripts");
    let out_dir = Path::new(&out_dir);

    fs::write(out_dir.join("posts_generated.rs"), posts_code)
        .expect("generated posts should be writable to OUT_DIR");
    fs::write(out_dir.join("content_generated.rs"), content_code)
        .expect("generated content should be writable to OUT_DIR");
}