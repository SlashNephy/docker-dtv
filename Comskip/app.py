import asyncio
import os
from subprocess import Popen

import aiohttp

async def main():
    while True:
        await asyncio.sleep(30)

        async with aiohttp.ClientSession() as session:
            async with session.get("http://chinachu:20772/api/recorded.json") as response:
                data = await response.json()

                for program in reversed(data):                
                    recorded = program["recorded"]

                    await handle(recorded)
                    await asyncio.sleep(5)

async def handle(recorded):
    txt_path = recorded.replace(".m2ts", ".txt")
    if os.path.exists(txt_path) or not os.path.exists(recorded):
        return

    with Popen(["comskip", "-t", "--ini=/etc/comskip.ini", recorded]) as p:
        p.wait()

if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
