import datetime

from dotenv import load_dotenv, set_key
from elf import get_puzzle_input

load_dotenv(".env.advc")
now = datetime.datetime.now()
days = 12
if now.year == 2025 and now.month == 12:
    days = min(now.day, days)

for i in range(1, days + 1):
    r = get_puzzle_input(2025, i)
    with open(f"inputs/day{i}.txt", "w") as f:
        f.write(r)

import os

set_key(".env", "CWD", os.path.abspath(os.path.dirname(__file__)))
