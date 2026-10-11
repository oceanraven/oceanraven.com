use pulldown_cmark::{Event, Options, Parser};
use std::fs;
use std::path::Path;

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

fn render_markdown(markdown: &str) -> String {
    let options = Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_SMART_PUNCTUATION
        | Options::ENABLE_MATH;

    let parser = Parser::new_ext(markdown, options);

    let events = parser.map(|event| match event {
        Event::InlineMath(tex) => Event::InlineHtml(render_math(&tex, false).into()),
        Event::DisplayMath(tex) => Event::InlineHtml(render_math(&tex, true).into()),
        other => other,
    });

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, events);
    html
}

fn render_math(tex: &str, display: bool) -> String {
    let option = katex::Opts::builder().display_mode(display).build().expect("Katex option build failed.");
    let result = katex::render_with_opts(tex, &option);
    result.unwrap_or_else(|e|
        panic!("invalid TeX {tex:?}: {e}")
    )
}

fn main() {
    println!("cargo:rerun-if-changed=posts");

    let mut posts = Vec::new();
    for entry in fs::read_dir("posts").expect("failed to read posts directory") {
        let path = entry.expect("failed to read directory entry").path();
        if !path.is_file() || path.extension().unwrap_or_default() != "md" {
            continue;
        }

        let slug = path
            .file_stem()
            .expect("post file has no name")
            .to_string_lossy()
            .into_owned();
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{}: failed to read: {e}", path.display()));

        let (frontmatter, body) = parse_frontmatter(&source, &path);
        let html = render_markdown(body);

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

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is not set");
    fs::write(Path::new(&out_dir).join("posts_generated.rs"), code)
        .expect("failed to write generated posts");
}