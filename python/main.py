import sys
import importlib
from time import time
from typing import Callable
from helpers.helpers import get_input_file, get_input_data

def run_part(part_func: Callable[[str], int], data: str, part_name: str) -> None:
    try:
        start_time = time()
        result = part_func(data)
        end_time = time()
        print(f"{part_name} result: {result}")
        print(f"Run time: {(end_time - start_time) * 1000 :0.6f} ms")
    except Exception as e:
        print(f"Error running {part_name}: {e}")


def main() -> None:
    if len(sys.argv) < 2:
        print("Usage: python3 main.py <day_number> <test>")
        sys.exit(1)

    day = sys.argv[1]
    input_file = get_input_file(day)
    input_data = get_input_data(input_file)

    try:
        day_module = importlib.import_module(f"solutions.day_{day}")
    except ModuleNotFoundError:
        print(f"No solution implemented for day {day}")
        sys.exit(1)

    try:
        day_module.test()
    except AttributeError:
        print(f"No tests implemented for day {day}")
    if len(sys.argv) == 3: # Ghetto but using for now
        return

    run_part(day_module.part_one, input_data, f"Day {day} Part 1")
    run_part(day_module.part_two, input_data, f"Day {day} Part 2")


if __name__ == "__main__":
    main()
