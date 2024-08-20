from concurrent.futures import Future, ThreadPoolExecutor
from typing import List, Optional

from .ddginternal import (
    PlacesModule,
    RecipesModule,
    assign_nrj_instances,
    get_nrj_instances,
)
from .primp import Client
from .utils import raise_for_status

native_modules_interpretation = {"places": "maps_places", "recipes": "recipes"}
native_modules = list(native_modules_interpretation.values())


class Module:
    """Represents a module collector (for result)."""

    def __init__(
        self,
        *,
        places: Optional[PlacesModule] = None,
        recipes: Optional[RecipesModule] = None,
    ):
        self.places = places
        self.recipes = recipes


def _gather(*futures: Future):
    return [future.result() for future in futures]


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

        elif who == "recipes":
            results[who] = assignee.recipes()

    return results
