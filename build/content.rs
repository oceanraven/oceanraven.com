use crate::markdown::Renderer;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Publication {
    title: String,
    venue: String,
    kind: String,
    authors: Vec<String>,
    year: u32,

    // links
    pdf: Option<String>,
    arxiv: Option<String>,
    code: Option<String>,
    slides: Option<String>,
    video: Option<String>,
}

#[derive(Deserialize)]
struct Publications {
    #[serde(default)]
    publications: Vec<Publication>,
}

pub fn generate(renderer: &Renderer) -> String {
    let research_path = "content/research.md";
    let research_content =
        fs::read_to_string(research_path).unwrap_or_else(|e| panic!("{research_path}: {e}"));
    let research_html = renderer.render(&research_content);

    let mut code = format!("pub static RESEARCH_HTML: &str = {:?};\n", research_html);

    let publications_path = "content/publications.toml";
    let publications_content = fs::read_to_string(publications_path)
        .unwrap_or_else(|e| panic!("{publications_path}: {e}"));
    let mut publications = toml::from_str::<Publications>(&publications_content)
        .unwrap_or_else(|e| panic!("{publications_path}: {e}"))
        .publications;
    publications.sort_by(|a, b| b.year.cmp(&a.year));

    code.push_str("pub static PUBLICATIONS: &[Publication] = &[\n");
    for publication in &publications {
        let links: Vec<(&str, &str)> = [
            ("pdf", &publication.pdf),
            ("arxiv", &publication.arxiv),
            ("code", &publication.code),
            ("slides", &publication.slides),
            ("video", &publication.video),
        ]
        .into_iter()
        .filter_map(|(label, link)| link.as_ref().map(|url| (label, url.as_str())))
        .collect();

        code.push_str(&format!(
            "    Publication {{ title: {:?}, authors: &{:?}, venue: {:?}, year: {}, kind: {:?}, links: &{:?} }},\n",
            publication.title,
            publication.authors,
            publication.venue,
            publication.year,
            publication.kind,
            links,
        ));
    }
    code.push_str("];\n");

    code
}
