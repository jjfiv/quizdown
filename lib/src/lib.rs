use parsing::QParser;
use pulldown_cmark::{Event, Options, Parser};
use std::fs;
use std::io;

#[macro_use]
extern crate serde_derive;

pub mod moodlexml;
mod parsing;

#[derive(Debug, thiserror::Error)]
pub enum Error {
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
    #[error("Content ignored after options!")]
    ContentIgnored,
    #[error("Internal Assertion Error")]
    Internal,
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

pub fn process_questions_str(content: &str) -> Result<Vec<Question>, Error> {
    let mut output = Vec::new();

    let mut md_opt = Options::empty();
    md_opt.insert(Options::ENABLE_STRIKETHROUGH);
    md_opt.insert(Options::ENABLE_TABLES);
    md_opt.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(content, md_opt);

    let tokens = parser.collect::<Vec<Event>>();
    let mut qp = QParser::new(tokens);

    while let Some(chunk) = qp.parse_next()? {
        output.push(chunk.finish());
    }

    Ok(output)
}

pub fn process_questions_file(path: &str) -> Result<Vec<Question>, Error> {
    let contents = fs::read_to_string(path)?;
    process_questions_str(&contents)
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
        let qs = process_questions_str(simple_q).unwrap();
        assert_eq!(1, qs.len());
    }

    #[test]
    fn test_broken_q() {
        let broken_q = r#"
## Who let the dogs out?

- I did it.
- Who, who, who?
        "#;
        let qs = process_questions_str(broken_q).unwrap_err();
        match qs {
            Error::NoOptionsFound => {}
            other => panic!("Expected NoOptionsFound error, got {:?}", other),
        }
    }
}
