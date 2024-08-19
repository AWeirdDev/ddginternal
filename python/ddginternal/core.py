from concurrent.futures import Future, ThreadPoolExecutor
from typing import Dict, List, Literal, Optional, Tuple, Union, overload

from .ddginternal import (
    Result,
    get_djs,
    get_result_binding,
    get_nrj_instances,
    assign_nrj_instances,
)
from .primp import Client, Response
from .module import Module

native_modules_interpretation = {"places": "maps_places"}
native_modules = list(native_modules_interpretation.values())


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


def get_module(client: Client, name: str) -> str:
    res = client.get("https://start.duckduckgo.com" + name)
    raise_for_status(res)
    return res.text


def load_module_from_djs_concurrently(
    djs: str,
    *,
    allowed_native_modules: List[str] = native_modules,
):
    pool = ThreadPoolExecutor()

    client = Client(impersonate="chrome_127", verify=False)

    instances = []
    module_futures = []
    for url, instance in get_nrj_instances(djs):
        if instance not in allowed_native_modules:
            continue

        instances.append(instance)
        module_futures.append(pool.submit(get_module, client, url))

    results = {}
    for assignee in assign_nrj_instances(
        list(zip(_gather(*module_futures), instances))
    ):
        who = assignee.who()
        if who == "places":
            results[who] = assignee.places()

    return results


def get_result(html: str, djs: str) -> Result:
    return get_result_binding(html, djs)


@overload
def search(q: str, *, modules: List[Literal["places"]]) -> Tuple[Result, Module]: ...


@overload
def search(q: str) -> Result: ...


def _gather(*futures: Future):
    return [future.result() for future in futures]


def search(
    q: str, *, modules: Optional[List[Literal["places"]]] = None
) -> Union[Result, Tuple[Result, Module]]:
    res = organic_search(q)

    if not modules:
        result = get_result(res["html"], res["djs"])
        return result

    executor = ThreadPoolExecutor()

    initial_task = executor.submit(get_result, res["html"], res["djs"])
    module_loader = executor.submit(
        load_module_from_djs_concurrently,
        res["djs"],
        allowed_native_modules=[
            native_modules_interpretation[name] for name in modules
        ],
    )
    module_loader_future = module_loader.result()

    return initial_task.result(), Module(places=module_loader_future.get("places"))
