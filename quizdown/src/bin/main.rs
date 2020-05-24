use quizdown::*;
use std::env;

fn main() -> Result<(), Error> {
    let mut questions: Vec<Question> = Vec::new();
    for path in env::args().skip(1) {
        let found = process_questions_file(&path)?;
        questions.extend(found.into_iter());
    }
    println!("{:?}", questions);
    Ok(())
}