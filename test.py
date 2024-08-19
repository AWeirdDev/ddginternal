from ddginternal import search

results = search("boba shops", modules=["places"])
print(results[1].places)
