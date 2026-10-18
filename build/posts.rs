use std::fs;
use std::path::Path;

use crate::markdown::Renderer;

#[derive(serde::Deserialize)]
struct Frontmatter {
    title: String,
    date: String,
    #[serde(default)]
    tags: Vec<String>,
}

struct Post {
    slug: String,
    frontmatter: Frontmatter,
    html: String,
}

pub fn generate(renderer: &Renderer) -> String {
    let mut posts = Vec::new();
    for entry in fs::read_dir("posts").expect("posts directory should exist at the project root") {
        let path = entry.expect("entries in posts directory should be readable").path();
        if !path.is_file() || path.extension().unwrap_or_default() != "md" {
            continue;
        }

        let slug = path
            .file_stem()
            .expect("post file should have a name")
            .to_string_lossy()
            .into_owned();
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{}: failed to read: {e}", path.display()));

        let (frontmatter, body) = parse_frontmatter(&source, &path);
        let html = renderer.render(body);

        posts.push(Post { slug, frontmatter, html });
    }
    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

    let mut code = String::from("pub static POSTS: &[Post] = &[\n");
    for post in &posts {
        code.push_str(&format!(
            "    Post {{ slug: {:?}, title: {:?}, date: {:?}, tags: &{:?}, html: {:?} }},\n",
            post.slug, post.frontmatter.title, post.frontmatter.date, post.frontmatter.tags, post.html,
        ));
    }
    code.push_str("];\n");
    code
}

fn split_frontmatter(post: &str) -> Option<(&str, &str)> {
    let rest = post.trim_start().strip_prefix("+++")?;
    let (raw, body) = rest.split_once("\n+++")?;
    Some((raw, body.trim_start()))
}

fn parse_frontmatter<'a>(post: &'a str, path: &Path) -> (Frontmatter, &'a str) {
    let (raw, body) = split_frontmatter(post).unwrap_or_else(|| {
        panic!("{}: post must start with a +++ frontmatter block", path.display())
    });
    let frontmatter = toml::from_str(raw)
        .unwrap_or_else(|e| panic!("{}: invalid frontmatter: {e}", path.display()));
    (frontmatter, body)
}