from typing import List, Self


class Point:
    def __init__(self, x: int, y: int):
        """
        For representing a point in 2D space
        In grids, x is the column and y is the row
        """
        self.x = x
        self.y = y

    def __str__(self) -> str:
        return f"({self.x}, {self.y})"

    def __repr__(self) -> str:
        return f"Point({self.x}, {self.y})"

    def all_neighbors(self) -> List[Self]:
        """
        Returns all 8 neighbors of the point
        """
        return [
            Point(self.x - 1, self.y - 1),
            Point(self.x - 1, self.y),
            Point(self.x - 1, self.y + 1),
            Point(self.x, self.y - 1),
            Point(self.x, self.y + 1),
            Point(self.x + 1, self.y - 1),
            Point(self.x + 1, self.y),
            Point(self.x + 1, self.y + 1)
        ]

    def __eq__(self, other) -> bool:
        return self.x == other.x and self.y == other.y

    def __lt__(self, other) -> bool:
        return self.y < other.y or (self.y == other.y and self.x < other.x)

    def __hash__(self) -> int:
        return hash((self.x, self.y))


class Grid:
    def __init__(self, grid: List[List[type]]):
        self.grid = grid
        self.width = len(grid[0])
        self.height = len(grid)

    def __str__(self) -> str:
        return "\n".join(["".join(row) for row in self.grid])

    def valid_point(self, p: Point) -> bool:
        return 0 <= p.x < self.width and 0 <= p.y < self.height

    def neighbors(self, p: Point) -> List[Point]:
        p_neighbors = p.all_neighbors()
        return [n for n in p_neighbors if self.valid_point(n)]
