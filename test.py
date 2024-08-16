import asyncio
from ddginternal import search


async def main():
    q = input("input > ")
    res = search(q)
    print(res)
    print(res.web[0])


asyncio.run(main())
