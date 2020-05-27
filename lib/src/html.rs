use crate::{Error, Question};
use std::fmt::Write;

pub fn render_html_preview(
    name: &str,
    questions: &[Question],
    full_page: bool,
) -> Result<String, Error> {
    let mut output = String::new();
    if full_page {
        output.push_str(
            "<html>
            <head>
                <title>Quizdown</title>
                <meta charset=\"utf-8\" />
            </head>
            <body>",
        );
    }
    writeln!(
        &mut output,
        "<i class='quizdown-loaded'>Loaded from {:?}</i>",
        name
    )?;
    for (i, q) in questions.iter().enumerate() {
        output.push_str("<div class='quizdown-question'>");
        writeln!(
            &mut output,
            "<div class='quizdown-prompt'>{}</div>",
            q.prompt
        )?;
        output.push_str(if q.ordered { "<ol>" } else { "<ul>" });
        for opt in &q.options {
            writeln!(
                &mut output,
                "<li class='quizdown-option'>
                    <input id='opt{}' type='checkbox' {} />
                    <label class='quizdown-label' for='opt{}'>{}</label>
                </li>",
                i,
                if opt.correct { "checked" } else { "" },
                i,
                opt.content
            )?;
        }
        output.push_str(if q.ordered { "</ol>" } else { "</ul>" });
        output.push_str("</div><hr />");
    }
    if full_page {
        output.push_str("</body></html>");
    }
    Ok(output)
}
