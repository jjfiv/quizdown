from .quizdown import lib, ffi
from typing import List, Optional, Dict, Any, Set
import json


def _rust_str(result) -> str:
    """
    Make a copy of a rust String and immediately free it!
    """
    try:
        txt = ffi.cast("char *", result)
        txt = ffi.string(txt).decode("UTF-8")
        return txt
    finally:
        assert lib.free_str(result)


def _raise_error_str(rust_error_string: Optional[str]):
    if rust_error_string is None:
        return
    if "{" in rust_error_string:
        response = json.loads(rust_error_string)
        if "error" in response and "context" in response:
            raise ValueError("{0}: {1}".format(response["error"], response["context"]))
    else:
        raise ValueError(rust_error_string)


def _handle_ffi_result(ffi_result):
    """
    This handles the logical-OR struct of the FFIResult { error_message, success }
    where both the wrapper and the error_message will be freed by the end of this function.

    The success pointer is returned or an error is raised!
    """
    if ffi_result == ffi.NULL:
        raise ValueError("FFIResult should not be NULL")

    error = None
    success = None
    if ffi_result.error_message != ffi.NULL:
        error = _rust_str(ffi_result.error_message)
    if ffi_result.success != ffi.NULL:
        success = ffi_result.success
    lib.free_ffi_result(ffi_result)

    # maybe crash here!
    if error is not None:
        _raise_error_str(error)
        return None

    # return the success pointer otherwise!
    return success


def available_themes() -> Set[str]:
    return set(_rust_str(lib.available_themes()).split("\t"))


def default_config() -> Dict[str, Any]:
    return json.loads(_rust_str(lib.default_config()))


AVAILABLE_FORMATS = ["HtmlSnippet", "HtmlFull", "MoodleXml"]


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
    return _rust_str(
        _handle_ffi_result(
            lib.parse_quizdown(
                input.encode("UTF-8"),
                name.encode("UTF-8"),
                of.encode("UTF-8"),
                config_str.encode("UTF-8"),
            )
        )
    )
