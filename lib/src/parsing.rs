use crate::render::SyntaxHighlighter;
use crate::{Error, QOption, Question};
use pulldown_cmark::{Event, Tag};
use std::fmt::Write;

#[derive(Debug)]
pub(crate) struct HeadingChunk<'md> {
    /// How many "#" were found.
    level: Option<u32>,
    /// The markdown after the "##" bit.
    header: Vec<Event<'md>>,
    /// Any markdown before the terminating task-list:
    contents: Vec<Event<'md>>,
    /// The parsed-out task-list:
    options: TaskList<'md>,
}

impl<'md> HeadingChunk<'md> {
    pub(crate) fn finish(self, renderer: &SyntaxHighlighter) -> Result<Question, Error> {
        let mut prompt = String::new();
        if let Some(lvl) = self.level {
            write!(prompt, "<h{}>", lvl).unwrap();
        } else {
            prompt.push_str("<b><i>");
        }
        renderer.render(&mut prompt, &self.header)?;
        if let Some(lvl) = self.level {
            write!(prompt, "</h{}>", lvl).unwrap();
        } else {
            prompt.push_str("</i></b>");
        }
        renderer.render(&mut prompt, &self.contents)?;
        let ordered = self.options.ordered;
        let options = self
            .options
            .question_options
            .into_iter()
            .map(|it| it.finish(renderer))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Question {
            prompt,
            ordered,
            options,
        })
    }
}

#[derive(Debug)]
struct TaskList<'md> {
    // ordered list?
    ordered: bool,
    question_options: Vec<TaskListOption<'md>>,
}

#[derive(Debug)]
struct TaskListOption<'md> {
    correct: bool,
    contents: Vec<Event<'md>>,
}
impl<'md> TaskListOption<'md> {
    fn finish(self, renderer: &SyntaxHighlighter) -> Result<QOption, Error> {
        let mut content = String::new();
        renderer.render(&mut content, &self.contents)?;
        Ok(QOption {
            correct: self.correct,
            content,
        })
    }
}

pub(crate) struct QParser<'md> {
    tokens: Vec<Event<'md>>,
    position: usize,
    list_stack: Vec<usize>,
}

impl<'md> QParser<'md> {
    pub(crate) fn new(tokens: Vec<Event<'md>>) -> Self {
        Self {
            tokens,
            position: 0,
            list_stack: Vec::new(),
        }
    }
    fn peek(&self) -> Option<Event<'md>> {
        self.tokens.get(self.position).cloned()
    }
    fn get(&mut self) -> Option<Event<'md>> {
        let out = self.tokens.get(self.position);
        if out.is_some() {
            self.position += 1;
        }
        out.cloned()
    }

    pub(crate) fn parse_next(&mut self) -> Result<Option<HeadingChunk<'md>>, Error> {
        let mut header = Vec::new();
        let mut contents = Vec::new();

        let here = self.get();
        if here.is_none() {
            return Ok(None);
        }
        let here = here.unwrap();
        let mut level: Option<u32> = None;

        match here {
            Event::Start(Tag::Heading(lvl)) => {
                level = Some(lvl);
            }
            _ => {}
        }
        // Process insides until end-heading of this level:
        if let Some(h) = level {
            loop {
                if let Some(next) = self.get() {
                    match &next {
                        Event::End(Tag::Heading(closed)) => {
                            debug_assert_eq!(*closed, h);
                            break;
                        }
                        _ => {}
                    }
                    // collect the parts that go in the heading.
                    header.push(next);
                } else {
                    return Err(Error::UnexpectedEOF(format!(
                        "while looking for end of heading h{}!",
                        h
                    )));
                }
            }
        } else {
            // otherwise, special "no-header" chunk.
            // un-get here
            self.position -= 1;
        }

        // Now, read the question body.
        // We expect to find 1 and only one "task_list".
        // We're done when:
        //  - we find the next header event.
        //  - we find an EOF.
        let start = self.position;
        let mut task_list_start: Option<usize> = None;
        let mut task_list_end: Option<usize> = None;
        loop {
            if let Some(next) = self.get() {
                match &next {
                    Event::End(Tag::List(_)) => {
                        let closed_list = self.list_stack.pop().unwrap();
                        if let Some(opened_list) = task_list_start {
                            if opened_list == closed_list {
                                task_list_end = Some(self.position);
                            }
                        }
                    }
                    Event::Start(tag) => match tag {
                        Tag::Heading(_) => {
                            // unget it for the next chunk.
                            self.position -= 1;
                            // stop looping.
                            break;
                        }
                        Tag::List(_) => {
                            // Remember this list!
                            self.list_stack.push(self.position - 1);
                        }
                        _ => {}
                    },
                    Event::TaskListMarker(_) => {
                        if self.list_stack.len() > 1 {
                            return Err(Error::NestedTaskList);
                        } else if self.list_stack.len() == 0 {
                            // This is probably impossible.
                            return Err(Error::TaskListWithoutList);
                        }
                        if let Some(prev) = task_list_start.replace(self.list_stack[0]) {
                            if prev != self.list_stack[0] {
                                return Err(Error::TooManyTaskLists);
                            }
                        }
                    }
                    _ => {}
                }
            } else {
                // here, EOF is OK!
                break;
            }
        }

        let end = self.position;
        // start..end is the question
        // task_list_start .. task_list_end is the options.

        let options = match (task_list_start, task_list_end) {
            (None, None) | (None, _) | (_, None) => {
                return Err(Error::NoOptionsFound);
            }
            (Some(t_start), Some(t_end)) => {
                if t_end != end {
                    return Err(Error::ContentIgnored);
                }
                contents.extend(self.tokens[start..t_start].iter().cloned());
                self.position = t_start;
                self.parse_task_list()?
            }
        };

        Ok(Some(HeadingChunk {
            level,
            header,
            contents,
            options,
        }))
    }

    fn parse_task_list_option(&mut self) -> Result<TaskListOption<'md>, Error> {
        let mut contents = Vec::new();

        match self.get() {
            Some(Event::Start(Tag::Item)) => {}
            x => panic!("expected list-item start, found: {:?}", x),
        };
        let correct = match self.get() {
            Some(Event::TaskListMarker(val)) => val,
            x => panic!("expected [_], found: {:?}", x),
        };

        loop {
            match self.get() {
                Some(Event::End(Tag::Item)) => break,
                Some(x) => contents.push(x),
                None => return Err(Error::Internal),
            };
        }

        Ok(TaskListOption { correct, contents })
    }

    fn parse_task_list(&mut self) -> Result<TaskList<'md>, Error> {
        // First better be a open_list:
        let ordered = match self.get() {
            Some(Event::Start(Tag::List(numbered))) => numbered.is_some(),
            _ => return Err(Error::Internal),
        };

        let mut question_options = Vec::new();

        loop {
            match self.peek() {
                Some(Event::End(Tag::List(_))) => {
                    let _ = self.get();
                    break;
                }
                Some(Event::Start(Tag::Item)) => {
                    question_options.push(self.parse_task_list_option()?)
                }
                None => break,
                other => panic!("Expected end-of-list or start-item; found {:?}", other),
            }
        }

        Ok(TaskList {
            ordered,
            question_options,
        })
    }
}
