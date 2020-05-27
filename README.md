# quizdown

Markdown, but for creating multiple-choice quizzes. Especially helpful if you routinely want syntax-highlighting in your question, answers, or distractors.

## What does it look like?

    ### Headings trigger new questions.

    That means you can put multiple questions in the same file.

    Easy code examples!
    ```python
    def take_default_list(xs = []):
        xs.append(7)
        return sum(xs)
    ```

    Mark the correct answer with a github task-list!
    What is the output of ``take_default_list([1,2,3])``?

    - [x] It's impossible to know.
    - [ ] 13
    - [ ] Something else.

## You can render it anywhere:
### Headings trigger new questions.

That means you can put multiple questions in the same file.

Easy code examples!
```python
def take_default_list(xs = []):
    xs.append(7)
    return sum(xs)
```

Mark the correct answer with a github task-list!
What is the output of ``take_default_list([1,2,3])``?

- [x] It's impossible to know.
- [ ] 13
- [ ] Something else.