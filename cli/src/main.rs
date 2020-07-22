use clap::{App, Arg};
use io::Write;
use quizdown_lib::*;
use std::fs::File;
use std::io;

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
            Arg::with_name("name")
            .long("--name")
                .value_name("QUIZ_NAME")
                .help("Some formats include the name of the course or quiz; by default this is merely your $INPUT_FILE.")
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
        .arg(
            Arg::with_name("theme")
            .long("--theme")
            .value_name("Syntax Highlighting Theme")
            .help("TODO, list some.")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("lang")
            .long("--lang")
            .value_name("Syntax Highlighting language for inline and unmarked code blocks.")
            .help("e.g., java, python, etc.")
            .takes_value(true)
        )
        .get_matches();

    let mut config = Config::default();
    if let Some(theme) = args.value_of("theme") {
        let available_themes = quizdown_lib::list_themes();
        if available_themes
            .iter()
            .map(|s| s.as_str())
            .filter(|t| t == &theme)
            .nth(0)
            .is_none()
        {
            eprintln!(
                "No such theme <{}>; try one of {:?}",
                theme, available_themes
            );
        }
        config.syntax.theme = theme.to_string();
    }
    if let Some(lang) = args.value_of("lang") {
        config.syntax.default_lang = lang.to_string();
    }

    let input = args
        .value_of("input")
        .expect("Input file name is required.");
    // read and process ASAP:
    let questions = process_questions_file(&input, Some(config))?;

    let name: &str = args.value_of("name").unwrap_or(input);

    let output_file_name = args.value_of("output").unwrap_or("-");
    let format: OutputFormat = match args.value_of("format") {
        None => {
            if output_file_name.ends_with(".html") {
                OutputFormat::HtmlFull
            } else if output_file_name.ends_with(".moodle") {
                OutputFormat::MoodleXml
            } else if output_file_name.ends_with(".json") {
                OutputFormat::JSON
            } else {
                panic!("Must provide a file format (--format=html) or an obvious output file e.g., '.html'");
            }
        }
        Some("html") => OutputFormat::HtmlFull,
        Some("json") => OutputFormat::JSON,
        Some("moodle") => OutputFormat::MoodleXml,
        Some(other) => panic!("Unknown format '{}'.", other),
    };

    let output = format.render(name, &questions)?;

    if output_file_name == "-" {
        println!("{}", output);
    } else {
        let mut fp = File::create(output_file_name)?;
        write!(fp, "{}", output)?;
    }

    Ok(())
}
