from . import quizdown_render, default_config, available_themes, AVAILABLE_FORMATS
import argparse
import os, sys
import webbrowser

parser = argparse.ArgumentParser(
    "quizdown", "quizdown MARKDOWN_FILE --output (out.moodle|preview.html)"
)
parser.add_argument(
    "input", metavar="INPUT_FILE", help="Markdown file containing quiz questions.",
)
parser.add_argument(
    "--output",
    metavar="OUTPUT_FILE",
    help="Where to save the processed output: a .moodle or .html extension.",
)
parser.add_argument(
    "--format",
    action="store",
    choices=AVAILABLE_FORMATS,
    help="Output format, if we cannot figure out from file extension.",
)
parser.add_argument(
    "--name",
    action="store",
    metavar="QUIZ_NAME",
    help="This is the name of the quiz or question category upon import. INPUT_FILE if not defined.",
)
config = default_config()
themes = available_themes()
parser.add_argument(
    "--theme",
    action="store",
    metavar="SYNTAX_THEME",
    choices=sorted(themes),
    default=config["syntax"]["theme"],
    help="Syntax highlighting-theme; default={}\n{} available.".format(
        config["syntax"]["theme"], themes
    ),
)
parser.add_argument(
    "--lang",
    action="store",
    default=config["syntax"]["default_lang"],
    help="Language string to assume for syntax-highlighting of un-marked code blocks; default='{}'; try 'python' or 'java'.".format(
        config["syntax"]["default_lang"]
    ),
)
parser.add_argument(
    "--browser",
    action="store_true",
    default=False,
    help="Directly open a preview in the default web-browser.",
)

args = parser.parse_args()

assert args.theme in themes
config["syntax"]["theme"] = args.theme
config["syntax"]["default_lang"] = args.lang

name = args.name or os.path.basename(args.input)
with open(args.input) as fp:
    raw_input = fp.read()

format = args.format
if not format:
    if not args.output:
        print("Must specify either --format or --output.")
        sys.exit(-1)
    (base, ext) = os.path.splitext(args.output)
    if ext == ".html":
        format = "HtmlFull"
    elif ext == ".json":
        format = "JSON"
    elif ext == ".moodle":
        format = "MoodleXml"
    else:
        raise ValueError(
            "Cannot guess format from '{}' for --output '{}'".format(ext, args.output)
        )

if args.browser and not args.output:
    raise ValueError("Must specify an --output html file to use --browser.")

render = quizdown_render(raw_input, name=name, format=format, config=config)
if args.output:
    with open(args.output, "w") as out:
        print(render, file=out)
    if format != "HtmlFull" and args.browser:
        raise ValueError("Use --format=HtmlFull with --browser.")
    if args.browser:
        path = os.path.abspath(args.output)
        webbrowser.open("file://" + path)
else:
    print(render)
