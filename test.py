from ddginternal import get_nrj_instances
from ddginternal.core import organic_search

search = organic_search("chocolate cookies")["djs"]
print(search)
print(get_nrj_instances(search))
