import ddginternal

result, mod = ddginternal.search("高田", modules=["places"])

if mod.places:
    result = mod.places.results[0]
    print(result.name)
    print(result.address)
    print(result.rating)
