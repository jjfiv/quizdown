use crate::Error;
use crate::Question;
use xmlwriter::*;

const TEXT_NODE: &str = "text";
const QUIZ_NODE: &str = "quiz";
const QUESTION_NODE: &str = "question";

pub fn to_moodle_xml(qs: &[Question], name: &str) -> Result<String, Error> {
    let opt = Options {
        ..Options::default()
    };
    let mut xml = XmlWriter::new(opt);
    xml.set_preserve_whitespaces(true);

    //
    //  <?xml version="1.0" ?>
    //  <quiz>
    //      <question type="category">
    //          <category>
    //              <text>$course$/XXXX</text>
    //          </category>
    //      </question>
    //      .
    //      .
    //  </quiz>
    //

    xml.write_declaration();
    xml.start_element(QUIZ_NODE);

    xml.start_element(QUESTION_NODE);
    xml.write_attribute("type", "category");
    xml.start_element("category");

    write_tag_str(&mut xml, "text", name);
    xml.end_element(); // </category>
    xml.end_element(); // </question>

    for (i, q) in qs.iter().enumerate() {
        write_multichoice(&mut xml, q, name, i)?;
    }

    // </quiz>
    xml.end_element();

    Ok(xml.end_document())
}

/// Write a single question to XML:
fn write_multichoice(
    xml: &mut XmlWriter,
    question: &Question,
    base_name: &str,
    index: usize,
) -> Result<(), Error> {
    let num_correct = question.options.iter().filter(|q| q.correct).count();
    let num_options = question.options.len();
    // Don't write questions that don't have options!
    if num_options == 0 {
        return Err(Error::NoOptionsFound);
    }
    if num_correct == 0 {
        return Err(Error::MoodleNoCorrectAnswer);
    }
    let correct_weight = 100.0 / (num_correct as f64);
    // use five-digits of precision:
    let correct_weight = format!("{:.5}", correct_weight);
    const INCORRECT_WEIGHT: &str = "-100";
    //<question type="multichoice">
    //<name><text>NAME</text></name>

    xml.start_element("question");
    xml.write_attribute("type", "multichoice");
    // <name><text>course/name/#</text></name>
    let name = format!("{}/{}", base_name, index);
    xml.start_element("name");
    write_tag_str(xml, TEXT_NODE, &name);
    xml.end_element(); // </name>

    //<questiontext format="html">
    //    <text>...</text>
    //</questiontext>
    xml.start_element("questiontext");
    xml.write_attribute("format", "html");
    write_tag_str(xml, TEXT_NODE, &question.prompt);
    xml.end_element(); // </questiontext>

    //<defaultgrade>1.0000000</defaultgrade>
    write_tag_str(xml, "defaultgrade", "1.0");

    //<answer fraction="33.33333" format="html">
    // <text>The correct answer</text>
    // <feedback><text>Correct!</text></feedback>
    //</answer>
    //<answer fraction="-100">
    //    ...
    //</answer>
    for ans in question.options.iter() {
        xml.start_element("answer");
        if ans.correct {
            xml.write_attribute("fraction", &correct_weight);
        } else {
            xml.write_attribute("fraction", INCORRECT_WEIGHT);
        }
        xml.write_attribute("format", "html");
        write_tag_str(xml, TEXT_NODE, &ans.content);

        xml.start_element("feedback");
        xml.start_element(TEXT_NODE);
        if ans.correct {
            xml.write_text("Correct!");
        } else {
            xml.write_text("Sorry, that's not correct!")
        }
        xml.end_element(); // </text>
        xml.end_element(); // </feedback>

        xml.end_element(); // answer
    }

    //<shuffleanswers>1</shuffleanswers>
    //<single>true</single>
    //<answernumbering>abc</answernumbering>
    write_tag_str(
        xml,
        "shuffleanswers",
        if question.ordered { "0" } else { "1" },
    );
    write_tag_str(xml, "single", "false");
    write_tag_str(xml, "answernumbering", "abc");

    //</question>
    xml.end_element(); // </question>
    Ok(())
}

/// Writes: <{tag}>{str}</{tag}>
fn write_tag_str(xml: &mut XmlWriter, tag: &str, contents: &str) {
    xml.start_element(tag);
    xml.write_text(contents);
    xml.end_element();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn encodes_correctly() {
        let mut expected = String::new();
        expected.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>");
        expected.push_str("<quiz><question type=\"category\"><category><text>cs101/ex</text></category></question>");
        expected.push_str("<question type=\"multichoice\"><name><text>cs101/ex/0</text></name><questiontext format=\"html\"><text>&lt;h2>Do you want to build a snowman?&lt;/h2></text></questiontext>");
        expected.push_str("<defaultgrade>1.0</defaultgrade>");
        expected.push_str("<answer fraction=\"-100\" format=\"html\"><text>No</text><feedback><text>Sorry, that\'s not correct!</text></feedback></answer>");
        expected.push_str("<answer fraction=\"100.00000\" format=\"html\"><text>Yes</text><feedback><text>Correct!</text></feedback></answer>");
        expected.push_str("<shuffleanswers>1</shuffleanswers><single>false</single><answernumbering>abc</answernumbering></question></quiz>");
        let question_bank = "cs101/ex";

        let q_src = r#"
## Do you want to build a snowman?

- [ ] No
- [x] Yes
        "#;
        let qs = process_questions_str(q_src, None).unwrap();
        let qxml = to_moodle_xml(&qs, question_bank).unwrap();
        println!("{}", qxml);
        assert_eq!(qxml, expected);
    }
}
