# Hello

`ddginternal` is a Python library for accessing DuckDuckGo's internal APIs through internal runtime Javascript parsing.

**Key features**:

- Keep your search private with [DuckDuckGo](https://html.duckduckgo.com).
- Works with Python 3.6+, and most [platforms](https://github.com/AWeirdDev/ddginternal/releases).
- Rust-backed, faster parsing.
- Simple API, flexible.
- Light. One dependency only.

```bash
$ pip install ddginternal
```

## Get started
To search, use the `search` function:

```python
import ddginternal

ddginternal.search("breaking bad")
# Result(web=[...23], images=[...20], news=[...5])
```

The numbers indicate the amount of results received for each category.
