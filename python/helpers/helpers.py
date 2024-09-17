import sys

def get_input_file(day: str):
    return f"../data/inputs/{day}.txt"

def get_input_data(file_path: str) -> str:
    try:
        with open(file_path) as f:
            return f.read()
    except FileNotFoundError:
        print("Input file for day {file_path} not found")
        sys.exit(1)
