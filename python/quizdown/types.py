import attr
from typing import List, Dict, Any, Optional


@attr.s
class QOption(object):
    correct: bool = attr.ib()
    content: str = attr.ib()
    uid: Optional[str] = attr.ib(default=None)

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "QOption":
        return QOption(d["correct"], d["content"])


@attr.s
class Question(object):
    prompt: str = attr.ib()
    ordered: bool = attr.ib()
    options: List[QOption] = attr.ib(factory=list)
    uid: Optional[str] = attr.ib(default=None)

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "Question":
        return Question(
            d["prompt"], d["ordered"], [QOption.from_dict(opt) for opt in d["options"]]
        )

    def option_uids(self) -> List[str]:
        return [opt.uid for opt in self.options if opt.uid]


@attr.s
class Quiz(object):
    name = attr.ib()
    questions: List[Question] = attr.ib(factory=list)
    uid: Optional[str] = attr.ib(default=None)

    def meta_id(self) -> str:
        return "meta:{}".format(self.uid)

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "Quiz":
        return Quiz(d["name"], [Question.from_dict(q) for q in d["questions"]])
