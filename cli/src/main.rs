use clap::{App, Arg};
use io::Write;
use quizdown::*;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io;

enum OutputFormat {
    Html,
    MoodleXml,
}

fn preview_html(path: &str, questions: &[Question]) -> Result<String, Error> {
    let mut output = String::new();
    output.push_str(
        "<html>
        <head>
            <title>Quizdown</title>
            <meta charset=\"utf-8\" />
        </head>
        <body>",
    );
    for q in questions {
        writeln!(
            &mut output,
            "<div class='question'><i>Loaded from {:?}</i>",
            path
        )?;
        writeln!(&mut output, "<div class='prompt'>{}</div>", q.prompt)?;
        output.push_str(if q.ordered { "<ol>" } else { "<ul>" });
        for opt in &q.options {
            writeln!(
                &mut output,
                "<li><label><input type='checkbox' />{}</label></li>",
                opt.content
            )?;
        }
        output.push_str(if q.ordered { "</ol>" } else { "</ul>" });
        output.push_str("</div><hr />");
    }
    output.push_str("</body></html>");
    Ok(output)
}

fn main() -> Result<(), Error> {
    let args = App::new("quizdown")
        .version("1.0")
        .about("Convert a markdown subset to formatted quiz questions.")
        .author("John Foley (johnf@middlebury.edu)")
        .arg(
            Arg::with_name("input")
                .value_name("INPUT_FILE")
                .takes_value(true)
                .help("Input markdown file."),
        )
        .arg(
            Arg::with_name("format")
            .long("--format")
                .short("-f")
                .value_name("FORMAT")
                .help("Output format: e.g., html, moodle")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("course")
            .long("--course")
                .value_name("quizdown")
                .help("Some formats include the name of the course; try: CSC212.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("quiz")
            .long("--quiz")
                .value_name("INPUT_FILE")
                .help("Some formats include the name of the quiz; same as your input file if not specified.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
            .long("--output")
                .short("-o")
                .value_name("OUTPUT_FILE")
                .help("Output file name; otherwise use stdout.")
                .takes_value(true),
        )
        .get_matches();

    let input = args
        .value_of("input")
        .expect("Input file name is required.");
    // read and process ASAP:
    let questions = process_questions_file(&input)?;

    let output_file_name = args.value_of("output").unwrap_or("-");
    let format: OutputFormat = match args.value_of("format") {
        None => {
            if output_file_name.ends_with(".html") {
                OutputFormat::Html
            } else if output_file_name.ends_with(".moodle") {
                OutputFormat::MoodleXml
            } else {
                panic!("Must provide a file format (--format=html) or an obvious output file e.g., '.html'");
            }
        }
        Some("html") => OutputFormat::Html,
        Some("moodle") => OutputFormat::MoodleXml,
        Some(other) => panic!("Unknown format '{}'.", other),
    };

    let course = args.value_of("course").unwrap_or("quizdown");
    let quiz = args.value_of("quiz").unwrap_or(input);

    let output = match format {
        OutputFormat::Html => preview_html(input, &questions)?,
        OutputFormat::MoodleXml => moodlexml::to_moodle_xml(questions, course, quiz)?,
    };

    if output_file_name == "-" {
        println!("{}", output);
    } else {
        let mut fp = File::create(output_file_name)?;
        write!(fp, "{}", output)?;
    }

    Ok(())
}
