use crate::Error;
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Tag};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{
    append_highlighted_html_for_styled_line, highlighted_html_for_string,
    start_highlighted_html_snippet, IncludeBackground,
};
use syntect::parsing::SyntaxSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxHighlightingOptions {
    pub theme: String,
    pub default_lang: String,
}

impl Default for SyntaxHighlightingOptions {
    fn default() -> Self {
        Self {
            default_lang: "text".to_owned(),
            theme: "InspiredGitHub".to_owned(),
        }
    }
}

impl SyntaxHighlightingOptions {
    pub(crate) fn create(&self) -> Result<SyntaxHighlighter, Error> {
        let ts = ThemeSet::load_defaults();
        if ts.themes.get(&self.theme).is_none() {
            return Err(Error::MissingSyntaxTheme(self.theme.clone()));
        }
        Ok(SyntaxHighlighter {
            ss: SyntaxSet::load_defaults_newlines(),
            ts,
            theme: self.theme.clone(),
            default_lang: self.default_lang.clone(),
        })
    }
}

pub(crate) struct SyntaxHighlighter {
    ss: SyntaxSet,
    ts: ThemeSet,
    theme: String,
    default_lang: String,
}

impl SyntaxHighlighter {
    fn highlight(&self, lang: &str, contents: &str) -> Result<String, Error> {
        let theme = &self.ts.themes[self.theme.as_str()];
        let syntax_ref = self
            .ss
            .find_syntax_by_token(lang)
            .unwrap_or_else(|| self.ss.find_syntax_plain_text());
        Ok(highlighted_html_for_string(
            contents, &self.ss, syntax_ref, theme,
        ))
    }
    fn highlight_inline(&self, contents: &str) -> Result<String, Error> {
        let theme = &self.ts.themes[self.theme.as_str()];
        let syntax_ref = self
            .ss
            .find_syntax_by_token(&self.default_lang)
            .unwrap_or_else(|| self.ss.find_syntax_plain_text());
        let mut highlighter = HighlightLines::new(syntax_ref, theme);
        let mut output = String::new();
        let (pre_start, bg) = start_highlighted_html_snippet(theme);
        debug_assert!(pre_start.starts_with("<pre style=\""));
        output.push_str("<code ");
        output.push_str(pre_start[4..].trim());

        let regions = highlighter.highlight(contents, &self.ss);
        append_highlighted_html_for_styled_line(
            &regions[..],
            IncludeBackground::IfDifferent(bg),
            &mut output,
        );
        output.push_str("</code>\n");
        Ok(output)
    }
    pub(crate) fn render<'a>(
        &self,
        output: &mut String,
        events: &[Event<'a>],
    ) -> Result<(), Error> {
        syntax_highlight_html(output, &self, events)
    }
}

fn syntax_highlight_html<'a>(
    output: &mut String,
    syntax: &SyntaxHighlighter,
    events: &[Event<'a>],
) -> Result<(), Error> {
    let mut with_highlight = Vec::with_capacity(events.len());
    let mut current_block_html = String::new();
    let mut i = 0;
    while i < events.len() {
        match &events[i] {
            Event::Start(Tag::CodeBlock(kind)) => {
                current_block_html.clear();
                let lang: &str = match kind {
                    CodeBlockKind::Indented => syntax.default_lang.as_ref(),
                    CodeBlockKind::Fenced(lang) => {
                        if lang.as_ref() == "" {
                            syntax.default_lang.as_ref()
                        } else {
                            lang.as_ref()
                        }
                    }
                };
                i += 1;
                while i < events.len() {
                    match &events[i] {
                        Event::End(Tag::CodeBlock(_)) => break,
                        Event::Text(line) => {
                            current_block_html.push_str(line.as_ref());
                        }
                        _ => panic!("Bad tag sequence."),
                    }
                    i += 1;
                }

                with_highlight.push(Event::Html(CowStr::from(
                    syntax.highlight(lang, &current_block_html)?,
                )));
            }
            Event::Code(inline) => {
                with_highlight.push(Event::Html(CowStr::from(syntax.highlight_inline(inline)?)))
            }
            other => {
                with_highlight.push(other.clone());
            }
        }
        i += 1;
    }

    html::push_html(output, with_highlight.into_iter());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pulldown_cmark::{Options, Parser};

    #[test]
    fn test_inline_in_para() {
        let contents = "What is the output of ``f1(3)``?";
        let mut md_opt = Options::empty();
        md_opt.insert(Options::ENABLE_STRIKETHROUGH);
        md_opt.insert(Options::ENABLE_TABLES);
        md_opt.insert(Options::ENABLE_TASKLISTS);
        let parser = Parser::new_ext(contents, md_opt);

        let tokens = parser.collect::<Vec<Event>>();
        println!("{:?}", tokens);

        let opts = SyntaxHighlightingOptions::default();
        let renderer = opts.create().unwrap();
        let mut html = String::new();
        renderer.render(&mut html, &tokens).unwrap();
        println!("html={}", html);
        assert_eq!("<p>What is the output of <code style=\"background-color:#ffffff;\"><span style=\"color:#323232;\">f1(3)</span></code>\n?</p>\n", html);
    }

    #[test]
    fn test_code_block_events() {
        let example = r#"
Hello! ``inline``.

```java
Java code here.
```

    indented code here
    and here.

Non-code here.
        "#;

        let mut md_opt = Options::empty();
        md_opt.insert(Options::ENABLE_STRIKETHROUGH);
        md_opt.insert(Options::ENABLE_TABLES);
        md_opt.insert(Options::ENABLE_TASKLISTS);
        let parser = Parser::new_ext(example, md_opt);

        let tokens = parser.collect::<Vec<Event>>();
        let mut in_code = false;
        let mut code_lines_found = Vec::new();
        for t in tokens {
            match t {
                Event::Start(Tag::CodeBlock(_)) => in_code = true,
                Event::End(Tag::CodeBlock(_)) => in_code = false,
                Event::Html(_) => panic!("No html tags!"),
                Event::Text(contents) => {
                    if in_code {
                        code_lines_found.push(contents.to_string())
                    }
                }
                Event::Code(inline) => assert_eq!(inline.as_ref(), "inline"),
                _ => continue,
            }
        }
        assert_eq!(
            vec!["Java code here.\n", "indented code here\n", "and here.\n"],
            code_lines_found
        );
    }
}
