use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use std::fs;
use std::path::Path;
use syntect::html::ClassStyle::SpacedPrefixed;
use syntect::html::{css_for_theme_with_class_style, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use two_face::theme::EmbeddedThemeName;

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

const SYNTAX_CLASS_PREFIX: &str = "syn-";

fn split_frontmatter(post: &str) -> Option<(&str, &str)> {
    let rest = post.trim_start().strip_prefix("+++")?;
    let (raw, body) = rest.split_once("\n+++")?;
    Some((raw, body.trim_start()))
}

fn parse_frontmatter<'a>(post: &'a str, path: &Path) -> (Frontmatter, &'a str) {
    let (raw, body) = split_frontmatter(post).unwrap_or_else(|| {
        panic!(
            "{}: post must start with a +++ frontmatter block",
            path.display()
        )
    });
    let frontmatter = toml::from_str(raw)
        .unwrap_or_else(|e| panic!("{}: invalid frontmatter: {e}", path.display()));
    (frontmatter, body)
}

fn highlight(code: &str, lang: &str, syntax_set: &SyntaxSet) -> String {
    let syntax = syntax_set
        .find_syntax_by_token(lang)
        .unwrap_or(syntax_set.find_syntax_plain_text());
    let mut generator = ClassedHTMLGenerator::new_with_class_style(
        syntax,
        syntax_set,
        SpacedPrefixed {
            prefix: SYNTAX_CLASS_PREFIX,
        },
    );
    for line in LinesWithEndings::from(code) {
        generator
            .parse_html_for_line_which_includes_newline(line)
            .unwrap_or_else(|e| panic!("syntect should highlight {lang:?} code: {e}"));
    }

    format!("<pre><code>{}</code></pre>\n", generator.finalize())
}

fn render_markdown(markdown: &str, syntax_set: &SyntaxSet) -> String {
    let options = Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_SMART_PUNCTUATION
        | Options::ENABLE_MATH;

    let parser = Parser::new_ext(markdown, options);

    let mut result: Vec<Event> = vec![];
    let mut code_language = None;
    let mut code_buffer = String::new();
    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(language)) => {
                code_language = match language {
                    CodeBlockKind::Indented => Some(String::new()),
                    CodeBlockKind::Fenced(info) => {
                        let lang = info
                            .split(|c: char| c == ',' || c.is_whitespace())
                            .next()
                            .unwrap_or("");
                        Some(lang.to_string())
                    }
                }
            }
            Event::Text(text) if code_language.is_some() => {
                code_buffer.push_str(&text);
            }
            Event::End(TagEnd::CodeBlock) => {
                let html = highlight(
                    &std::mem::take(&mut code_buffer),
                    code_language.take().as_deref().unwrap_or(""),
                    syntax_set,
                );
                result.push(Event::Html(html.into()));
            }
            Event::InlineMath(tex) => {
                result.push(Event::InlineHtml(render_math(&tex, false).into()))
            }
            Event::DisplayMath(tex) => {
                result.push(Event::InlineHtml(render_math(&tex, true).into()))
            }
            other => result.push(other),
        };
    }

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, result.into_iter());
    html
}

fn render_math(tex: &str, display: bool) -> String {
    let option = katex::Opts::builder()
        .display_mode(display)
        .build()
        .expect("KaTeX display options should always be valid");
    let result = katex::render_with_opts(tex, &option);
    result.unwrap_or_else(|e| panic!("invalid TeX {tex:?}: {e}"))
}

fn main() {
    println!("cargo:rerun-if-changed=posts");

    let mut posts = Vec::new();

    let syntax_set = two_face::syntax::extra_newlines();

    for entry in fs::read_dir("posts").expect("posts directory should exist at the project root") {
        let path = entry
            .expect("entries in posts directory should be readable")
            .path();
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
        let html = render_markdown(body, &syntax_set);

        posts.push(Post {
            slug,
            frontmatter,
            html,
        });
    }
    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

    let mut code = String::from("pub static POSTS: &[Post] = &[\n");
    for post in &posts {
        code.push_str(&format!(
            "    Post {{ slug: {:?}, title: {:?}, date: {:?}, tags: &{:?}, html: {:?} }},\n",
            post.slug,
            post.frontmatter.title,
            post.frontmatter.date,
            post.frontmatter.tags,
            post.html,
        ));
    }
    code.push_str("];\n");

    let themes = two_face::theme::extra();
    let css = css_for_theme_with_class_style(
        &themes[EmbeddedThemeName::OneHalfDark],
        SpacedPrefixed {
            prefix: SYNTAX_CLASS_PREFIX,
        },
    )
    .expect("syntect should generate CSS for a built-in theme");
    code.push_str(&format!("pub const SYNTAX_CSS: &str = {css:?};\n"));

    let out_dir =
        std::env::var("OUT_DIR").expect("OUT_DIR should be set by Cargo for build scripts");
    fs::write(Path::new(&out_dir).join("posts_generated.rs"), code)
        .expect("generated posts should be writable to OUT_DIR");
}
