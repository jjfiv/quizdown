use parsing::QParser;
use pulldown_cmark::{Event, Options, Parser};
use std::fs;
use std::{io, ops::Range};
use syntect::highlighting::ThemeSet;

#[macro_use]
extern crate serde_derive;

pub mod html;
pub mod moodlexml;
mod parsing;
mod render;
pub use render::SyntaxHighlightingOptions;

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Copy)]
pub enum OutputFormat {
    /// Render HTML with title, body, etc. for file preview.
    HtmlFull,
    /// Render just the HTML for the questions themselves.
    HtmlSnippet,
    /// MoodleXML import format.
    MoodleXml,
}

impl OutputFormat {
    pub fn render(&self, name: &str, questions: &[Question]) -> Result<String, Error> {
        Ok(match self {
            OutputFormat::HtmlFull | OutputFormat::HtmlSnippet => {
                html::render_html_preview(name, questions, self == &OutputFormat::HtmlFull)?
            }
            OutputFormat::MoodleXml => moodlexml::to_moodle_xml(questions, name)?,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Format-Err {0:?}")]
    Fmt(#[from] std::fmt::Error),
    #[error("IO {0:?}")]
    IO(#[from] io::Error),
    #[error("Unexpected-EOF {0}")]
    UnexpectedEOF(String),
    #[error("Unexpected {0}")]
    Unexpected(String),
    #[error("Non-uint points {0}")]
    PointsParseErr(#[from] std::num::ParseIntError),
    #[error("Nested list options are not supported.")]
    NestedTaskList,
    #[error("Internal: TaskList event without List event?")]
    TaskListWithoutList,
    #[error("Found multiple lists with options; not supported.")]
    TooManyTaskLists,
    #[error("Found no options in question.")]
    NoOptionsFound,
    #[error("Moodle requires correct answers for every question!")]
    MoodleNoCorrectAnswer,
    #[error("Content ignored after options!")]
    ContentIgnored,
    #[error("Internal Assertion Error")]
    Internal,
    #[error("Missing Syntax theme: '{0}'")]
    MissingSyntaxTheme(String),
    #[error("Missing Syntax for language: '{0}'")]
    MissingSyntaxLang(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub insert_none_of_the_above: bool,
    pub syntax: SyntaxHighlightingOptions,
}

#[derive(Serialize, Debug, Clone)]
pub struct QOption {
    pub correct: bool,
    pub content: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Question {
    pub prompt: String,
    pub options: Vec<QOption>,
    pub ordered: bool,
}

pub fn list_themes() -> Vec<String> {
    let ts = ThemeSet::load_defaults();
    let mut themes = ts.themes.keys().cloned().collect::<Vec<String>>();
    themes.sort_unstable();
    themes
}

pub fn process_questions_str(
    content: &str,
    config: Option<Config>,
) -> Result<Vec<Question>, Error> {
    let mut output = Vec::new();
    let config = config.unwrap_or_default();
    let highlighter = config.syntax.create()?;

    let mut md_opt = Options::empty();
    md_opt.insert(Options::ENABLE_STRIKETHROUGH);
    md_opt.insert(Options::ENABLE_TABLES);
    md_opt.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(content, md_opt);
    let mut qp = QParser::new(parser);

    while let Some(chunk) = qp.parse_next()? {
        output.push(chunk.finish(&highlighter)?);
    }

    Ok(output)
}

pub fn process_questions_file(path: &str, config: Option<Config>) -> Result<Vec<Question>, Error> {
    let contents = fs::read_to_string(path)?;
    process_questions_str(&contents, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple_q() {
        let simple_q = r#"
## Who let the dogs out?

- [ ] I did it.
- [x] Who, who, who?
        "#;
        let qs = process_questions_str(simple_q, None).unwrap();
        assert_eq!(1, qs.len());
    }

    #[test]
    fn test_broken_q() {
        let broken_q = r#"
## Who let the dogs out?

- I did it.
- Who, who, who?
        "#;
        let qs = process_questions_str(broken_q, None).unwrap_err();
        match qs {
            Error::NoOptionsFound => {}
            other => panic!("Expected NoOptionsFound error, got {:?}", other),
        }
    }
}
