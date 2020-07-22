from . import quizdown_to_py, default_config, available_themes, AVAILABLE_FORMATS
import zipfile
import argparse
import os, json
import uuid

from jinja2 import Template

"""
The next bit of template magic from: https://stackoverflow.com/a/20885799
"""
try:
    import importlib.resources as pkg_resources
except ImportError:
    # Try backported to PY<37 `importlib_resources`.
    import importlib_resources as pkg_resources
from . import templates  # relative-import the *package* containing the templates

QTI_QUIZ_TMPL = Template(pkg_resources.read_text(templates, "qti-quiz.j2"))
QTI_QUIZ_META_TMPL = Template(pkg_resources.read_text(templates, "qti-quiz-meta.j2"))
QTI_MANIFEST_TMPL = Template(pkg_resources.read_text(templates, "qti-manifest.j2"))


def make_id() -> str:
    return str(uuid.uuid4())


if __name__ == "__main__":
    parser = argparse.ArgumentParser("quizdown_qti", "quizdown_qti MARKDOWN_FILE")
    parser.add_argument(
        "inputs",
        metavar="INPUT_FILES",
        help="Markdown files containing quiz questions.",
        nargs="+",
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
    args = parser.parse_args()
    quizzes = []
    for path in args.inputs:
        with open(path) as fp:
            raw_input = fp.read()
        base_file = os.path.basename(path)
        name = os.path.splitext(base_file)[0]
        quiz = quizdown_to_py(raw_input, name)
        # give every quiz, option & question a UUID
        quiz.uid = quiz.name
        for question in quiz.questions:
            question.uid = make_id()
            for opt in question.options:
                opt.uid = make_id()
        quizzes.append(quiz)

    with zipfile.ZipFile("output.qti.zip", "w") as zf:
        zf.writestr(
            "imsmanifest.xml",
            QTI_MANIFEST_TMPL.render(
                ident="TBD", title="Quizdown Import TODO", quizzes=quizzes
            ),
        )

        for q in quizzes:

            quiz_xml = QTI_QUIZ_TMPL.render(quiz=q, title=q.name, questions=q.questions)
            quiz_zip_path = "{0}/{0}.xml".format(q.uid)
            zf.writestr(quiz_zip_path, quiz_xml)

            meta_zip_path = "{0}/assessment_meta.xml".format(q.uid)
            meta_xml = QTI_QUIZ_META_TMPL.render(
                quiz=q,
                title=q.name.replace("_", " "),
                points_possible=float(len(q.questions)),
                assignment_identifier=make_id(),
            )
            zf.writestr(meta_zip_path, meta_xml)

