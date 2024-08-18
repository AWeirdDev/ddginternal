# Basics
`ddginternal` is based on the internal APIs thus it involves many function calls that may be hard to understand for beginners. To use a higher-level API, you can directly use the `search()` function:

```python
from ddginternal import search

result = search("chocolate")
```

This returns a `Result` object (ported from Rust) that contains a list of web results, images, news articles, and abstract information (i.e. Wikipedia insights).

For a quick overview, we'll get the very first result of each category:

```python
print(result.web[0], 
      result.images[0], 
      result.news[0], 
      result.abstract, 
      sep="\n\n")
```

```python
Web(raw_description="<b>Chocolate</b> is one of the most popular food types and flavors in the world, and many foodstuffs involving <b>chocolate</b> exist, particularly desserts, including cakes, pudding, mousse, <b>chocolate</b> brownies, and <b>chocolate</b> chip cookies. Many candiesare filled with or coated with sweetened <b>chocolate</b>.", description="Chocolate is one of the most popular food types and flavors in the world, and many foodstuffs involving chocolate exist, particularly desserts, including cakes, pudding, mousse, chocolate brownies, and chocolate chip cookies. Many candiesare filled with or coated with sweetened chocolate.\n", domain="en.wikipedia.org", shortened_url="en.wikipedia.org/wiki/Chocolate", url="https://en.wikipedia.org/wiki/Chocolate", title="Chocolate - Wikipedia")  

Image(height=1536, width=1041, image="https://elavegan.com/wp-content/uploads/2023/01/homemade-chocolate-bars-1041x1536.jpg", thumbnail="https://tse1.mm.bing.net/th?id=OIP.m-FNyduBkQUp_LsS0L5nEgHaK7&pid=Api", title="How To Make Chocolate (3 Ingredients) - Elavegan", url="https://elavegan.com/how-to-make-chocolate/")

NewsArticle(date=1723926123, excerpt="The viral Olympic Village <b>chocolate</b> muffins have made their New York City debut with hundreds lining up at the East Village pop-up to get a taste. Hundreds of people lined up on 17 August to check out the new pop-up dedicated to the <b>chocolate</b> muffins that were all the rage in the Olympic Village at the 2024 Paris Olympic Games after Norwegian swimmer Henrik Christiansen posted about them on TikTok.", image="https://media.zenfs.com/en/the_independent_635/4ab32f23d488fec6af06851b786d5166", relative_time="17 hours ago", source="The Independent on MSN.com", title="'Muffin mania': Hundreds line up to try the viral Olympic Village chocolate muffins in New York City", url="https://www.msn.com/en-gb/travel/news/muffin-mania-hundreds-line-up-to-try-the-viral-olympic-village-chocolate-muffins-in-new-york-city/ar-AA1oYlF3")

Abstract(text="Chocolate or cocoa is a food made from roasted and ground cocoa seed kernels that is available as a liquid, solid, or paste, either on its own or as a flavoring agent in other foods. Cocoa has been consumed in some form for at least 5,300 years starting with the Mayo-Chinchipe culture in what is present-day Ecuador. Later Mesoamerican civilizations also consumed chocolate beverages, and it was introduced to Europe in the 16th century. The seeds of the cacao tree have an intense bitter taste and must be fermented to develop the flavor. After fermentation, the seeds are dried, cleaned, and roasted. The shell is removed to produce nibs, which are then ground to cocoa mass, unadulterated chocolate in rough form. Once the cocoa mass is liquefied by heating, it is called chocolate liquor. The liquor may also be cooled and processed into its two components: cocoa solids and cocoa butter.", source="Wikipedia", url="https://en.wikipedia.org/wiki/Chocolate", answer="", definition="", entity="food", heading="Chocolate", image="/i/f5810b7016c536ea.jpg", infobox=[Infobox(label="\"Region or state\"", value="\"Mesoamerica\""), Infobox(label="\"Main ingredients\"", value="\"Cocoa bean\""), Infobox(label="\"Variations\"", value="\"Chocolate liquor, cocoa butter, cocoa solids, solid chocolate\""), Infobox(label="\"Wikidata description\"", value="\"nutritionally dense or sweet food product from the seed of Theobroma cacao - cocoa bean\""), Infobox(label="\"Wikidata id\"", value="\"Q195\""), Infobox(label="\"Wikidata label\"", value="\"chocolate\""), Infobox(label="\"Wikidata aliases\"", value="[\"chocolate\"]")])
```

You can then get each result of a category, and use it everywhere in your apps.
