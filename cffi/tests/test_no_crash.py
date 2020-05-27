import unittest

from quizdown import *
import bs4
from bs4 import BeautifulSoup

EXAMPLE = """
## What is your favorite color?

 - [x] red
 - [ ] blue
 - [ ] green
"""


def soupify(html):
    return BeautifulSoup(html, "html.parser")


class TestNoCrash(unittest.TestCase):
    def test_themes(self):
        themes = available_themes()
        self.assertNotEqual(0, len(themes))

    def test_config(self):
        config = default_config()
        themes = available_themes()
        self.assertTrue(config["syntax"]["theme"] in themes)

    def test_render_html(self):
        content = quizdown_render(EXAMPLE, format="HtmlSnippet")
        tags = soupify(content)
        correct = tags.select("input[checked]")
        self.assertEqual(1, len(correct))
        correct_id = correct[0]["id"]
        found = False
        for lbl in tags.select("label"):
            if lbl["for"] == correct_id:
                found = True
                self.assertEqual(lbl.text.strip(), "red")
        self.assertTrue(found)
