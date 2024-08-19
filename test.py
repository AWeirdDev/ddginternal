from ddginternal import get_nrj_instances, load_module_from_djs_concurrently
from ddginternal.core import organic_search

search = organic_search("chocolate cookies")["djs"]
# print(search)
print(get_nrj_instances(search))

print(load_module_from_djs_concurrently(search, allowed_native_modules=["recipes"]))
