import asyncio
import os

import psutil
from aiohttp import ClientSession
from discord import AsyncWebhookAdapter, Embed, Webhook

async def main():
    last_percents = {}

    while True:
        await handle(last_percents)
        await asyncio.sleep(30)

async def handle(last_percents):
    url = os.environ.get("DISCORD_WEBHOOK_URL")
    if not url:
        return

    threshold_percent = to_int_or_none(os.environ.get("LOW_SPACE_THRESHOLD_PERCENT"))
    threshold_gb = to_int_or_none(os.environ.get("LOW_SPACE_THRESHOLD_GB"))
    
    if not threshold_percent and not threshold_gb:
        return

    giga = 2.0 ** 30
    for disk in os.listdir("/mnt"):
        path = f"/mnt/{disk}"
        if not os.path.ismount(path):
            continue

        usage = psutil.disk_usage(path)
        free_percent = 100 - usage.percent
        total_gb = usage.total / giga
        free_gb = (usage.total - usage.used) / giga

        print(f"{path}: {free_gb} / {total_gb} GB ({free_percent} %)")

        if (threshold_percent and free_percent <= threshold_percent) or (threshold_gb and free_gb <= threshold_gb):
            # 1% 以内の変動のときは無視する
            if disk in last_percents and last_percents[disk] - 1 <= free_percent <= last_percents[disk]:
                pass
            else:
                await post_webhook(url, disk, free_percent, free_gb, total_gb)

        last_percents[disk] = free_percent

def to_int_or_none(value):
    try:
        return int(value)
    except:
        return None

async def post_webhook(url, disk, free_percent, free_gb, total_gb):
    embed = Embed(
        description="ストレージの空き容量が不足しています！",
        color=0xd42222
    )
    embed.add_field(name="ドライブ", value=f"/mnt/{disk}", inline=False)
    embed.add_field(name="空き容量", value=f"{round(free_gb, 1)} / {round(total_gb, 1)} GB ({round(free_percent, 1)} %)", inline=False)

    async with ClientSession(raise_for_status=True) as session:
        webhook = Webhook.from_url(url, adapter=AsyncWebhookAdapter(session))

        await webhook.send(
            username="Storage Watcher",
            embed=embed
        )

if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
