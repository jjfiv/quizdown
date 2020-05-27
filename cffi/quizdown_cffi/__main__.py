from . import quizdown_render

# TODO implement cli in python.
test = """
## Is python working?

- [x] Yes.
- [ ] No.
- [ ] I hope so?...
"""
print(quizdown_render(test, format="HtmlFull"))
