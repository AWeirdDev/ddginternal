import ddginternal

while True:
    print(
        ddginternal.load_module_from_djs(
            ddginternal.organic_search(input(">> "))["djs"]
        )
    )
