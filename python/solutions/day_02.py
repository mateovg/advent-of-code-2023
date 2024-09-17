import sys
import math
import itertools
import functools
import collections
from functools import lru_cache, cache, cmp_to_key
from helpers.helpers import get_input_data

RED = 12
GREEN = 13
BLUE = 14


def part_one(input: str) -> int:
    return sum(
        map(
            lambda x: x["id"],
            filter(
                valid_game,
                map(
                    process,
                    input.splitlines())
            )
        )
    )


def part_two(input: str) -> int:
    return sum(
        map(
            power_set,
            map(
                process,
                input.splitlines()
            )
        )
    )


def process(line: str) -> dict:
    line_split = line.split(":")
    game_id = line_split[0].split()[1]
    games = line_split[1].split(";")

    game_dict = {
        "id": int(game_id),
        "red": set(),
        "green": set(),
        "blue": set(),
    }

    for game in games:
        for game_split in game.split(","):
            roll, color = game_split.split()
            game_dict[color].add(int(roll))
    # print(game_dict)
    return game_dict


def valid_game(game: dict) -> bool:
    red = max(game["red"])
    green = max(game["green"])
    blue = max(game["blue"])

    if red > RED:
        return False
    if green > GREEN:
        return False
    if blue > BLUE:
        return False

    return True


def fewest_cubes(game: dict) -> (int, int, int):
    red = max(game["red"])
    green = max(game["green"])
    blue = maxt (game["blue"])

    return red, green, blue


def power_set(game: dict) -> int:
    r, g, b = fewest_cubes(game)
    print(f"Powerset of: {game['id']} - {r} {g} {b} - {r * g * b}")
    return r * g * b


def test() -> None:
    day = __file__.split("/")[-1][4:6]
    test_input_path = f"../data/examples/{day}.txt"
    test_data = get_input_data(test_input_path)

    print(f"Part one: {part_one(test_data)}")
    assert part_one(test_data) == 8
    print(f"Part two: {part_two(test_data)}")
    assert part_two(test_data) == 2286
    print("All tests passed")
