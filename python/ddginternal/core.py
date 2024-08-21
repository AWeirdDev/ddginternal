from concurrent.futures import ThreadPoolExecutor
from typing import Dict, List, Literal, Optional, Tuple, Union, overload

from .ddginternal import (
    Result,
    get_djs,
    get_result_binding,
)
from .modules import (
    Module,
    load_module_from_djs_concurrently,
    native_modules_interpretation,
)
from .primp import Client
from .utils import raise_for_status

ModuleNames = Literal["places", "recipes"]
user_agent = (
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) "
    "Chrome/125.0.0.0 Safari/537.36 OPR/111.0.0.0"
)

def organic_search(q: str, **kwargs) -> Dict[Literal["html", "djs"], str]:
    """Gets the contents of `d.js` and the HTML.

    Example:
    .. code-block:: python

        data = organic_search("chocolate")
        data['html'], data['djs']

    Args:
        q (str): The search query.
        **kwargs: Additional parameters to be passed to the search query, allowing for fine-tuning of the search.

    Returns:
        dict["html" | "djs", str]: The contents of `d.js` and the HTML.
    """
    client = Client(impersonate="chrome_127", verify=False)

    # Get the start page (simpler search, possibly fewer ads)
    payload = {"q": q}.update(kwargs)
    if not payload:
        payload = {"q": q}
    res = client.get("https://start.duckduckgo.com", params=payload)
    raise_for_status(res)

    # The /d.js file stores all information about the search results
    djs_url = get_djs(res.text)

    djs_res = client.get("https://links.duckduckgo.com" + djs_url)
    raise_for_status(djs_res)

    return {"html": res.text, "djs": djs_res.text}


def get_result(html: str, djs: str) -> Result:
    return get_result_binding(html, djs)


@overload
def search(q: str, *, modules: List[ModuleNames]) -> Tuple[Result, Module]:
    """Search with additional modules.

    Example:
    .. code-block:: python

        result, mod = search("boba shop", modules=["places"])
        print(result.web[0])
        print(mod.places)

    Args:
        q (str): The query.
        modules (list[ModuleNames]): The modules to enable.

    Returns:
        tuple[Result, Module]: The first item being the result expected when using `search()`
            normally and the second one being contents of the module selected.
    """


@overload
def search(q: str) -> Result:
    """Search with DuckDuckGo (no modules).

    Example:
    .. code-block:: python

        result = search("chocolate")
        print(result.abstract)

    Args:
        q (str): The query.

    Returns:
        Result: The result.
    """


def search(
    q: str, *, modules: Optional[List[ModuleNames]] = None, **kwargs
) -> Union[Result, Tuple[Result, Module]]:
    """Search with DuckDuckGo.

    .. note::
        See the documentation of each overload for more information.

    Args:
        q (str): The query.
        modules (Optional[list[ModuleNames]]): The modules to enable. Default is None.
        **kwargs: Additional parameters to be passed to the search query, allowing for fine-tuning of the search.

    Returns:
        Result | tuple[Result, Module]: The result or the contents of the modules selected.
    """

    res = organic_search(q, **kwargs)

    if not modules:
        result = get_result(res["html"], res["djs"])
        return result

    executor = ThreadPoolExecutor()

    with ThreadPoolExecutor() as executor:
        initial_task = executor.submit(get_result, res["html"], res["djs"])
        module_loader = executor.submit(
            load_module_from_djs_concurrently,
            res["djs"],
            allowed_native_modules=[
                native_modules_interpretation[name] for name in modules
            ],
        )
        result = initial_task.result()
        module_loader_future = module_loader.result()

    return result, Module(
        places=module_loader_future.get("places"),
        recipes=module_loader_future.get("recipes"),
    )
