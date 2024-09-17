import sys
import math
import itertools
import functools
import collections
from functools import lru_cache, cache, cmp_to_key
from helpers.helpers import get_input_data

def part_one(input: str) -> int:
    return 0

def part_two(input: str) -> int:
    return 0

def test() -> None:
    day = __file__.split("/")[-1][4:6]
    test_input_path = f"../data/examples/{day}.txt"
    test_data = get_input_data(test_input_path)

    assert part_one(test_data) == 0
    assert part_two(test_data) == 0
    print("All tests passed")


