import sys
import math
import itertools
import functools
import collections
from functools import lru_cache, cache, cmp_to_key, reduce
from helpers.helpers import get_input_data
from helpers.ds import Point, Grid
from typing import List


def part_one(input: str) -> int:
    grid = process(input)
    symbols = find_symbols(grid)
    print(f"Found {len(symbols)} symbols")

    parts = find_numbers(grid)
    print(f"Found {len(parts)} parts")


    print(f"Found {len(parts)} valid parts")

    return sum(map(lambda x: x.get("number", 0), reduce(lambda a, b: a + b, parts, [])))


def part_two(input: str) -> int:
    return 0


def process(input: str) -> Grid:
    grid = list(
        map(
            lambda x: list(x),
            input.splitlines()
        )
    )
    grid = Grid(grid)
    return grid


def find_symbols(grid: Grid) -> List[List[dict]]:
    # Keep symbols in their rows
    grid = grid.grid
    if not grid:
        return []
    symbols = []
    for row in range(len(grid)):
        new_row = []
        for col in range(len(grid[row])):
            if grid[row][col].isalpha() and grid[row][col] != ".":
                new_col.append({"point": Point(col, row), "symbol": grid[row][col]})
        symbols.append(new_row[:])
    return symbols


def find_numbers(grid) -> List[List[dict]]:
    grid = grid.grid
    numbers = []
    for row in range(len(grid)):
        col = 0
        num = 0
        new_row = []
        while col < len(grid[row]):
            if grid[row][col].isdigit():
                num = num * 10 + int(grid[row][col])
            elif num != 0:
                num_digits = math.floor(math.log10(num)) + 1
                points = set()
                for d in range(num_digits):
                    points.add(Point(col - d - 1, row))
                new_row.append({"number": num, "points": points})
                num = 0
            col += 1
    return numbers


def is_part(number: int, points: set[dict[Point, chr]], symbols: set[dict[Point, chr]]) -> bool:
    return False


def test(input: str) -> None:
    grid = process(input)
    symbols = list(find_symbols(grid))
    assert len(symbols) == 6

    numbers = dict(find_numbers(grid))

    assert part_one(input) == 4361
    print("Part one test passed")
    assert part_two(input) == 0
    print("All tests passed")
