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
        "<div class='question'><i>Loaded from {:?}</i>",
        name
    )?;
    for q in questions {
        writeln!(&mut output, "<div class='prompt'>{}</div>", q.prompt)?;
        output.push_str(if q.ordered { "<ol>" } else { "<ul>" });
        for opt in &q.options {
            writeln!(
                &mut output,
                "<li><label>
                    <input type='checkbox' />{}</label></li>",
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
