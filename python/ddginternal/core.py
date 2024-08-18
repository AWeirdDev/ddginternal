from typing import Dict, Literal

from .ddginternal import Result, get_djs, get_result_binding
from .primp import Client, Response


def raise_for_status(res: Response):
    assert res.status_code == 200, res.text


def organic_search(q: str) -> Dict[Literal["html", "djs"], str]:
    """Gets the contents of `d.js` and the HTML.

    Example:
    .. code-block:: python

        data = organic_search("chocolate")
        data['html'], data['djs']

    Args:
        q (str): The search query.

    Returns:
        dict["html" | "djs", str]: The contents of `d.js` and the HTML.
    """
    client = Client(impersonate="chrome_127", verify=False)

    # Get the start page (simpler search, possibly fewer ads)
    res = client.get("https://start.duckduckgo.com", params={"q": q})
    raise_for_status(res)

    # The /d.js file stores all information about the search results
    djs_url = get_djs(res.text)

    djs_res = client.get("https://links.duckduckgo.com" + djs_url)
    raise_for_status(djs_res)

    return {"html": res.text, "djs": djs_res.text}


def get_results(html: str, djs: str) -> Result:
    return get_result_binding(html, djs)


def search(q: str) -> Result:
    res = organic_search(q)
    return get_results(res["html"], res["djs"])
