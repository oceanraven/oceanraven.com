use crate::markdown::Renderer;
use serde::Deserialize;
use std::cmp::Reverse;
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

#[derive(Deserialize)]
struct Project {
    name: String,
    summary: String,
    year: u32,
    status: Status,
    #[serde(default)]
    tags: Vec<String>,

    repo: Option<String>,
    demo: Option<String>,
    docs: Option<String>,

    post: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Status {
    Active,
    Wip,
    Archived,
}

impl Status {
    fn rank(&self) -> u32 {
        match self {
            Status::Active => 0,
            Status::Wip => 1,
            Status::Archived => 2,
        }
    }
}

#[derive(Deserialize)]
struct Projects {
    #[serde(default)]
    projects: Vec<Project>,
}

fn collect_links<'a>(candidates: &[(&'a str, &'a Option<String>)]) -> Vec<(&'a str, &'a str)> {
    candidates
        .iter()
        .filter_map(|(label, link)| link.as_ref().map(|url| (*label, url.as_str())))
        .collect()
}

pub fn generate(renderer: &Renderer, post_slugs: &[String]) -> String {
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
        let links = collect_links(&[
            ("pdf", &publication.pdf),
            ("arxiv", &publication.arxiv),
            ("code", &publication.code),
            ("slides", &publication.slides),
            ("video", &publication.video),
        ]);

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

    let projects_path = "content/projects.toml";
    let projects_content =
        fs::read_to_string(projects_path).unwrap_or_else(|e| panic!("{projects_path}: {e}"));
    let mut projects = toml::from_str::<Projects>(&projects_content)
        .unwrap_or_else(|e| panic!("{projects_path}: {e}"))
        .projects;

    projects.sort_by_key(|p| (p.status.rank(), Reverse(p.year)));

    code.push_str("pub static PROJECTS: &[Project] = &[\n");
    for project in &projects {
        if let Some(slug) = &project.post {
            if !post_slugs.contains(slug) {
                panic!("projects.toml: {:?} links to missing post {:?}", project.name, slug)
            }
        }

        let links = collect_links(&[
            ("repo", &project.repo),
            ("demo", &project.demo),
            ("docs", &project.docs),
        ]);

        code.push_str(&format!(
            "    Project {{ name: {:?}, summary: {:?}, year: {}, status: Status::{:?}, tags: &{:?}, links: &{:?}, post: {:?} }},\n",
            project.name,
            project.summary,
            project.year,
            project.status,
            project.tags,
            links,
            project.post,
        ));
    }
    code.push_str("];\n");

    code
}
