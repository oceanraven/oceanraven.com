use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use syntect::html::{css_for_theme_with_class_style, ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use two_face::theme::EmbeddedThemeName;

const CLASS_STYLE: ClassStyle = ClassStyle::SpacedPrefixed { prefix: "syn-" };
const THEME: EmbeddedThemeName = EmbeddedThemeName::OneHalfDark;

pub struct Renderer {
    syntax_set: SyntaxSet,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            syntax_set: two_face::syntax::extra_newlines(),
        }
    }

    pub fn render(&self, markdown: &str) -> String {
        let options = Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TABLES
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_SMART_PUNCTUATION
            | Options::ENABLE_MATH;

        let mut events = Vec::new();
        let mut code_language: Option<String> = None;
        let mut code_buffer = String::new();

        for event in Parser::new_ext(markdown, options) {
            match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    code_language = Some(match kind {
                        CodeBlockKind::Indented => String::new(),
                        CodeBlockKind::Fenced(info) => info
                            .split(|c: char| c == ',' || c.is_whitespace())
                            .next()
                            .unwrap_or("")
                            .to_string(),
                    });
                }
                Event::Text(text) if code_language.is_some() => {
                    code_buffer.push_str(&text);
                }
                Event::End(TagEnd::CodeBlock) => {
                    let language = code_language.take().unwrap_or_default();
                    let html = self.highlight(&std::mem::take(&mut code_buffer), &language);
                    events.push(Event::Html(html.into()));
                }
                Event::InlineMath(tex) => {
                    events.push(Event::InlineHtml(render_math(&tex, false).into()));
                }
                Event::DisplayMath(tex) => {
                    events.push(Event::InlineHtml(render_math(&tex, true).into()));
                }
                other => events.push(other),
            }
        }

        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, events.into_iter());
        html
    }

    fn highlight(&self, code: &str, lang: &str) -> String {
        let syntax = self
            .syntax_set
            .find_syntax_by_token(lang)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let mut generator =
            ClassedHTMLGenerator::new_with_class_style(syntax, &self.syntax_set, CLASS_STYLE);
        for line in LinesWithEndings::from(code) {
            generator
                .parse_html_for_line_which_includes_newline(line)
                .unwrap_or_else(|e| panic!("syntect should highlight {lang:?} code: {e}"));
        }

        format!("<pre><code>{}</code></pre>\n", generator.finalize())
    }
}

pub fn syntax_css() -> String {
    let theme_set = two_face::theme::extra();
    css_for_theme_with_class_style(&theme_set[THEME], CLASS_STYLE)
        .expect("syntect should generate CSS for a built-in theme")
}

fn render_math(tex: &str, display: bool) -> String {
    let options = katex::Opts::builder()
        .display_mode(display)
        .build()
        .expect("KaTeX display options should always be valid");
    katex::render_with_opts(tex, &options)
        .unwrap_or_else(|e| panic!("invalid TeX {tex:?}: {e}"))
}