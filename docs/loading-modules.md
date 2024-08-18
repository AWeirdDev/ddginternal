# Loading Modules
!!! warning
    This page is still under construction. The API is subject to change.

Duckduckgo searches use modules to load a variety of search categories on-demand. You can load a module from the response `d.js` using `load_module_from_djs()`:

```python
from ddginternal import load_module_from_djs, organic_search

djs = organic_search("boba shop")['djs']
load_module_from_djs(djs)
```
Currently, only the `PlacesModule` is supported.