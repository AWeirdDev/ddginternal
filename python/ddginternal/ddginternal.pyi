"""Duckduckgo internal APIs from Rust."""

from typing import Any, List, Optional

def get_djs(html: str) -> str:
    """Gets `d.js` from the initial HTML.

    Args:
        html (str): The HTML.

    Raises:
        RegexError: If the regex fails to compile.
    """

def get_result_binding(html: str, djs: str):
    """Gets the result binding from HTML and `d.js`.

    See :py:func:`ddginternal.organic_search`

    Args:
        html (str): The HTML.
        djs (str): The `d.js` content.
    """

def get_embedded_abstract(html: str) -> str:
    """Gets the embedded abstract from `d.js`.

    Args:
        html (str): The `d.js` content.

    Raises:
        RegexError: If the regex fails to compile.
    """

def get_nrj_instances(djs: str) -> List[tuple[str, str]]: ...
def assign_nrj_instances(instances: List[tuple[str, str]]) -> List["Assignee"]: ...

class Assignee:
    def who(self) -> str: ...
    def places(self) -> Any: ...

class Result:
    web: List["_Web"]
    images: List["_Image"]
    news: List["_NewsArticle"]
    abstract: Optional[object]

class _Web:
    raw_description: str
    description: str
    domain: str
    shortened_url: str
    url: str
    title: str

class _Image:
    url: str
    title: str
    description: str
    thumbnail: str
    width: int
    height: int

class _NewsArticle:
    date: int
    excerpt: str
    title: str
    url: str
    source: str
    relative_time: str
    image: Optional[str]

class RegexError(Exception):
    """Represents a regex error."""
