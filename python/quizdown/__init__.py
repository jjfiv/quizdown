import quizdown.quizdown as lib
from .types import Quiz, Question, QOption
from typing import List, Optional, Dict, Any, Set
import json


def available_themes() -> Set[str]:
    return set(lib.available_themes())


def default_config() -> Dict[str, Any]:
    return json.loads(lib.default_config())


AVAILABLE_FORMATS = ["HtmlSnippet", "HtmlFull", "MoodleXml", "JSON"]


def quizdown_render(
    input: str, name: str = "quizdown", format: str = "MoodleXml", config=None
) -> str:
    """
    Parse some quizdown text into a sequence of questions.

    raises: ValueError
    """
    if config is None:
        config = default_config()

    config_str = ""
    if type(config) is dict:
        config_str = json.dumps(config)
    elif type(config) is str:
        config_str = config
    else:
        raise ValueError(config)

    if format not in AVAILABLE_FORMATS:
        raise ValueError(format)
    of = json.dumps({format: None})
    return lib.try_parse_quizdown(input, name, of, config_str)
    


def quizdown_to_py(input: str, name: str) -> Quiz:
    """
    Parse some quizdown text into a Quiz object.

    raises: ValueError
    """
    return Quiz.from_dict(json.loads(quizdown_render(input, name, "JSON")))
