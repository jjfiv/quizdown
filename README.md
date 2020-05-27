# quizdown [![PyPI version](https://badge.fury.io/py/quizdown.svg)](https://pypi.org/project/quizdown)

Markdown, but for creating multiple-choice quizzes. Especially helpful if you routinely want syntax-highlighting in your question, answers, or distractors.

Try a [live version (beta)](https://static.jjfoley.me/quizdown-web/) on my website.

## What is this?

This is a tool for quickly specifying 5-20 multiple choice questions in a markdown subset. Right now you can export to both MoodleXML and HTML. Blackboard seems to be a big deal at my new job, so import support for that is coming soon.

I also used this format with a lecture-quiz system of my own design -- that's coming soon.

## Why would I use this over Moodle's built-in editor?

- Less clicks! Make as many questions as you want with just your keyboard. Then import them in bulk to a "Question Bank" and then from there to a new "Quiz".
- You teach CS/Data Science/STEM and you want or NEED some ***good*** syntax highlighting for your class.
- Sane defaults: all questions are "select as many as apply", with no partial credit.

## Limitations

 - ONLY Multiple choice questions are supported.
 - Any partial credit must be done post-export via Moodle.
 - No way to upload images. You could theoretically embed SVG and base64 images but I haven't looked into it.
 - Error messages are limited (I just figured out how to get position information from the markdown library; need to sprinkle it through). For now, treat it like LaTeX: binary search for your errors ;p

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

## You can render it anywhere:

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