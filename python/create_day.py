import os
import shutil
import sys

def create_day(day_number: str) -> None:
    template_file = "solutions/day_x.py"
    new_day_file = f"solutions/day_{day_number}.py"

    if os.path.exists(new_day_file):
        print(f"Day {day_number} already exists")
        sys.exit(1)


    shutil.copy(template_file, new_day_file)
    print(f"Created new solution file: {new_day_file}")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage: python3 create_day.py <day_number>")
        sys.exit(1)

    day_number = sys.argv[1]
    create_day(day_number)
