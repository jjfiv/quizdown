use quizdown::*;
use std::env;

fn main() -> Result<(), Error> {
    println!("<html>
    <head>
    <title>Quizdown</title>
    <meta charset=\"utf-8\" />
    </head>
    <body>");
    for path in env::args().skip(1) {
        let found = process_questions_file(&path)?;

        for q in found {
            println!("<div class='question'><i>Loaded from {:?}</i>", path);
            println!("<div class='prompt'>{}</div>", q.prompt);
            if q.ordered {
                println!("<ol>");
            } else {
                println!("<ul>");
            }
            for opt in q.options {
                println!("<li><label><input type='checkbox' />{}</label></li>", opt.content)
            }
            if q.ordered {
                println!("</ol>");
            } else {
                println!("</ul>");
            }
            println!("</div>");
            println!("<hr />");
        }
    }
    println!("</body></html>");
    Ok(())
}