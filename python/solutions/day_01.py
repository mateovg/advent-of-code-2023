import sys
import math
import itertools
import functools
import collections
from functools import lru_cache, cache, cmp_to_key
from helpers.helpers import get_input_data

digit_map = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}

def part_one(input: str) -> int:
    return sum(process(line, 1) for line in input.splitlines())

def part_two(input: str) -> int:
    return sum(process(line, 2) for line in input.splitlines())

def process(input: str, part: int) -> int:
    digits = []
    if part == 1:
        digits = list(filter(lambda x: x.isdigit(), input))
    if part == 2:
        for i in range(len(input)):
            if input[i].isdigit():
                digits.append(input[i])
                continue
            for digit in digit_map.keys():
                if input.startswith(digit, i):
                    digits.append(digit_map[digit])

    if len(digits) == 0:
        print("No digits found")
        return 0

    left = int(digits[0])
    right = int(digits[-1])
    number = left * 10 + right

    # print(f"{input} -> {digits} -> {number}")
    return number

def test() -> None:
    day = __file__.split("/")[-1][4:6]
    test_input_path = f"../data/examples/{day}.txt"
    test_data = get_input_data(test_input_path)

    part_one_res = part_one(test_data)
    print(f"Part one test: {part_one_res}")
    assert part_one_res == 209

    part_two_res = part_two(test_data)
    print(f"Part two test: {part_two_res}")
    assert part_two_res == 281

    print("All tests passed")


