use pulldown_cmark::{Options, Parser};

const POSTS: &[(&str, &str)] = &[
    ("test1", include_str!("../posts/test1.md")),
    ("test2", include_str!("../posts/test2.md")),
];

#[derive(serde::Deserialize, Debug)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

pub struct Post {
    pub slug: String,
    pub frontmatter: Frontmatter,
    pub body: String,
}

fn parse_post(slug: &str, post: &str) -> Post {
    let (frontmatter, body) = parse_frontmatter(post);
    Post {
        slug: slug.to_string(),
        frontmatter,
        body: body.to_string(),
    }
}

pub fn all_posts() -> Vec<Post> {
    let mut all_posts = POSTS
        .iter()
        .map(|(slug, post)| parse_post(slug, post))
        .collect::<Vec<Post>>();
    all_posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));
    all_posts
}

pub fn find_post(slug: &str) -> Option<Post> {
    all_posts()
        .into_iter()
        .find(|post| post.slug == slug)
}

fn split_frontmatter(post: &str) -> Option<(&str, &str)> {
    let rest = post.trim_start().strip_prefix("+++")?;
    let (raw, body) = rest.split_once("\n+++")?;
    Some((raw, body.trim_start()))
}

fn parse_frontmatter(post: &str) -> (Frontmatter, &str) {
    let (raw, body) =
        split_frontmatter(post).expect("post must start with a +++ frontmatter block");
    let frontmatter = toml::from_str(raw).expect("invalid frontmatter");
    (frontmatter, body)
}

pub fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(markdown, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}
