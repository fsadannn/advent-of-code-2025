import datetime
import json
import os
import pathlib

from dotenv import load_dotenv, set_key
from elf import get_puzzle_input

load_dotenv(".env.advc")
now = datetime.datetime.now()
days = 12
if now.year == 2025 and now.month == 12:
    days = min(now.day, days)


cache_file = "inputs/cache.json"


def get_cache():
    try:
        with open(cache_file, "r", encoding="utf-8") as f:
            return json.load(f)
    except FileNotFoundError:
        return {}


def set_cache(cache):
    with open(cache_file, "w", encoding="utf-8") as f:
        json.dump(cache, f)


def should_skip(i):
    cache = get_cache()
    file = pathlib.Path(f"inputs/day{i}.txt")
    if file.exists() and file.stat().st_mtime == cache.get(str(i)):
        return True
    return False


for i in range(1, days + 1):
    if should_skip(i):
        print(f"skipping day {i}")
        continue
    print(f"downloading day {i}")
    r = get_puzzle_input(2025, i)
    file = pathlib.Path(f"inputs/day{i}.txt")
    with open(file, "w", encoding="utf-8") as f:
        f.write(r)
    cache = get_cache()
    cache[str(i)] = file.stat().st_mtime
    print(f"caching day {i}")
    set_cache(cache)


set_key(".env", "CWD", os.path.abspath(os.path.dirname(__file__)))
