# quizdown [![PyPI version](https://badge.fury.io/py/quizdown.svg)](https://pypi.org/project/quizdown) ![Rust](https://github.com/jjfiv/quizdown/workflows/Rust/badge.svg)

Markdown, but for creating multiple-choice quizzes. Especially helpful if you routinely want syntax-highlighting in your question, answers, or distractors.

Try a [live version (beta)](https://static.jjfoley.me/quizdown-web/) on my website.

## Off-line Quick-Start

### Install (from PyPI)

Maybe in a virtualenv? This may need to be ``pip3``.

```bash
pip install quizdown
```

### Preview a Markdown file in the browser.

```bash
python -m quizdown 01_syllabus.md --output 01_syllabus.html --browser
```

### Export to Moodle:

```bash
# Use the .moodle extension
python -m quizdown 01_syllabus.md --output 01_syllabus.moodle
# If you'd rather .xml:
python -m quizdown 01_syllabus.md --format=moodle --output 01_syllabus.xml
```

### More options:

```
python -m quizdown --help
usage: quizdown MARKDOWN_FILE --output (out.moodle|preview.html)

positional arguments:
  INPUT_FILE            Markdown file containing quiz questions.

optional arguments:
  -h, --help            show this help message and exit
  --output OUTPUT_FILE  Where to save the processed output: a .moodle or .html
                        extension.
  --format {HtmlSnippet,HtmlFull,MoodleXml}
                        Output format, if we cannot figure out from file
                        extension.
  --name QUIZ_NAME      This is the name of the quiz or question category upon
                        import. INPUT_FILE if not defined.
  --theme SYNTAX_THEME  Syntax highlighting-theme; default=InspiredGitHub
                        {'Solarized (dark)', 'base16-ocean.light',
                        'base16-ocean.dark', 'base16-eighties.dark',
                        'Solarized (light)', 'InspiredGitHub',
                        'base16-mocha.dark'} available.
  --lang LANG           Language string to assume for syntax-highlighting of
                        un-marked code blocks; default='text'; try 'python' or
                        'java'.
  --browser             Directly open a preview in the default web-browser.
```

## What is ``quizdown``?

This is a tool for quickly specifying 5-20 multiple choice questions in a markdown subset. Right now you can export to both MoodleXML and HTML. 

## Why would I use this over Moodle's built-in editor?

- Less clicks! Make as many questions as you want with just your keyboard. Then import them in bulk to a "Question Bank" and then from there to a new "Quiz".
- You teach CS/Data Science/STEM and you want or NEED some ***good*** syntax highlighting for your class.
- Sane defaults: all questions are "select as many as apply", with no partial credit.

## Limitations

 - ONLY Multiple choice questions are supported.
 - Any partial credit must be done post-export via Moodle.
 - No way to upload images. You could theoretically embed SVG and base64 images but I haven't looked into it.
 - Error messages are limited (I just figured out how to get position information from the markdown library; need to sprinkle it through). For now, treat it like LaTeX: binary search for your errors ;p

## Roadmap

 - Other question types, e.g., Essay questions? #1
 - Better error messages. (No line/col or question # information right now) #2
 - QTI/Blackboard export support. #3
 - File an issue: https://github.com/jjfiv/quizdown/issues


## How to write a bunch of questions (in words):

 - Use headings (whatever level you want; be consistent) to separate questions.
 - Questions end with a github-style task list -- if you want moodle to shuffle, use unordered lists, otherwise make them ordered.
 - Tasks marked as "complete" are correct answers.
 - We're building on [pulldown_cmark](https://github.com/raphlinus/pulldown-cmark); a CommonMark-compatible markdown implementation with the "github tables" "github task lists" and "strikethrough" extensions.

## Example

### Source Question

Let's imagine we're teaching Python and want to make sure students (A) understand list-append, and (B) remember that lists should never be used as default arguments.

    ### A. Python Lists
    
    ```python
    xs = [1,2,3]
    xs.append(7)
    print(sum(xs))
    ```
    
    What gets printed?
    
    1. [ ] It's impossible to know.
    1. [x] 13
    1. [ ] Something else.


    ### B. Python Lists and Default Arguments

    ```python
    def take_default_list(xs = []):
        xs.append(7)
        return sum(xs)
    ```

    What is the output of ``take_default_list([1,2,3])``?

    1. [x] It's impossible to know.
    1. [ ] 13
    1. [ ] Something else.

## Example (rendered)

I have a private github repo for each class, with files labeled by lecture number and topic, e.g., ``05_Lists.md`` -- Any old Markdown renderer is close enough for 99% of questions.

Here's someone's README.md rendering of the above example questions.

### A. Python Lists

```python
xs = [1,2,3]
xs.append(7)
print(sum(xs))
```

What gets printed?

1. [ ] It's impossible to know.
1. [x] 13
1. [ ] Something else.


### B. Python Lists and Default Arguments

```python
def take_default_list(xs = []):
    xs.append(7)
    return sum(xs)
```

What is the output of ``take_default_list([1,2,3])``?

1. [x] It's impossible to know.
1. [ ] 13
1. [ ] Something else.

### Why'd you write this in Rust?

Because my first version (in Python w/BeautifulSoup) was a bit of a disaster, maintenance-wise. Also, I wanted to have the ability to host an editor online. So this one compiles to WASM.