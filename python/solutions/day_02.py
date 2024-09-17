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
    return sum(map(lambda x: x["id"], filter(lambda game: valid_game(**game), map(process, input.splitlines()))))

def part_two(input: str) -> int:
    return 0

def process(line: str) -> dict:
    line_split = line.split(":")
    game_id = line_split[0].split()[1]
    games = line_split[1].split(";")

    game_dict = {
        "id": int(game_id),
        "red": 0,
        "green": 0,
        "blue": 0
    }

    for game in games:
        for game_split in game.split(","):
            roll, color = game_split.split()
            game_dict[color] = max(game_dict[color], int(roll))

    print(game_dict)
    return game_dict

def valid_game(id: int, red: int, green: int, blue: int) -> bool:
    if red > RED: return False
    if green > GREEN: return False
    if blue > BLUE: return False
    return True


def test() -> None:
    day = __file__.split("/")[-1][4:6]
    test_input_path = f"../data/examples/{day}.txt"
    test_data = get_input_data(test_input_path)

    assert part_one(test_data) == 8
    assert part_two(test_data) == 0
    print("All tests passed")


