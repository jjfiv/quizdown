from . import quizdown_parse

# TODO implement cli in python.
test = """
## Is python working?

- [x] Yes.
- [ ] No.
- [ ] I hope so?...
"""
print(quizdown_parse(test))
