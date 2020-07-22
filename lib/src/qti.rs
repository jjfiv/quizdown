use crate::Error;
use crate::Question;
use xmlwriter::*;
use tera::*;


pub fn to_qti_quiz(qs: &[Question], name: &str) -> Result<String, Error> {

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

}