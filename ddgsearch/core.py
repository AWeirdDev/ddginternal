import json
import re
from typing import List, Optional, Tuple

from selectolax.lexbor import LexborHTMLParser

from .primp import Client


def organic_search(q: str) -> Tuple[List[dict], Optional[List[dict]]]:
    client = Client(impersonate="chrome_127", verify=False)

    # Get the start page (simpler search, possibly fewer ads)
    res = client.get("https://start.duckduckgo.com", params={"q": q})

    # The /d.js file stores all information about the search results
    target_d = re.findall(r"'(/d\.js\?.+)'", res.text)
    js = client.get("https://links.duckduckgo.com" + target_d[0]).text

    page_layout = (
        re.findall(r"""DDG\.pageLayout\.load\('d',(\[{.+}),{"n""", js)[0] + "]"
    )

    a_js = re.findall(r"nrj\('(/a.js\?.+?)',.+?\)", js)  # noqa: F841
    extensions = None

    if a_js:
        extensions = json.loads(
            re.findall(
                r"\((\[.+\])\)", client.get("https://duckduckgo.com" + a_js[0]).text
            )[0]
        )

    return json.loads(page_layout), extensions


def get_page_payload(payload: List[dict]):
    blocks = []

    for item in payload:
        # Abstract (aka. description)
        abstract_parser = LexborHTMLParser(item["a"])
        abstract_parser.unwrap_tags(["p", "a", "b", "strong"])
        abstract = abstract_parser.text(strip=True)

        # Link
        anchor = item["c"]

        blocks.append((anchor, abstract))

    return blocks


def get_extension(data: Optional[List[dict]]) -> Optional[dict]:
    if not data:
        return None

    return data[0]["data"][0]


def search(q: str) -> dict:
    o = organic_search(q)
    return {
        "results": get_page_payload(o[0]),
        "extension": get_extension(o[1]),
    }
