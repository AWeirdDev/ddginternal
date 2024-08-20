from ddginternal import search

result, mod = search("chocolate cookies", modules=["recipes"])

if mod.recipes:
    recipe = mod.recipes.results[0]
    print("#", recipe.title)
    print(recipe.summary)
    print("score:", recipe.spoonacular_score)
