from collections import Counter
import sys
from typing import Optional


MOVEMENT: list[list[tuple[int, int]]] = [
    [(0, -1), (-1, -1), (0, -1), (1, -1)],
    [(0, 1), (-1, 1), (0, 1), (1, 1)],
    [(-1, 0), (-1, -1), (-1, 0), (-1, 1)],
    [(1, 0), (1, -1), (1, 0), (1, 1)],
]

NEIGHBORS: list[tuple[int, int]] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 1),
    (1, -1), (1, 0), (1, 1),
]


class Grove:
    def __init__(self, lines: list[str]) -> None:
        self.elves: list[tuple[int, int]] = []
        self.occupied: set[tuple[int, int]] = set()
        for y, line in enumerate(lines):
            for x, c in enumerate(line):
                if c == '#':
                    self.elves.append((x, y))
                    self.occupied.add((x, y))

    def simulate(self, n) -> None:
        for i in range(n):
            self.step(i)

    def simulate_until_stop(self) -> int:
        i = 0
        while self.step(i):
            i += 1
        return i + 1

    def step(self, i: int) -> bool:
        proposed: list[Optional[tuple[int, int]]] = []
        for x, y in self.elves:
            p: Optional[tuple[int, int]] = None
            if self.occupied & {(x+dx, y+dy) for dx, dy in NEIGHBORS}:
                for j in range(len(MOVEMENT)):
                    (dx, dy), *check = MOVEMENT[(i + j) % len(MOVEMENT)]
                    if not (self.occupied & {(x+dx1, y+dy1) for dx1, dy1 in check}):
                        p = x+dx, y+dy
                        break
            proposed.append(p)
        if all(p is None for p in proposed):
            return False
        counter = Counter(p for p in proposed if p is not None)
        for i, p in enumerate(proposed):
            if p is not None and counter[p] == 1:
                self.occupied.remove(self.elves[i])
                self.elves[i] = p
                self.occupied.add(p)
        return True
        # self.draw()

    def draw(self) -> None:
        x0, y0, w, h = self.get_bounds()
        for y in range(y0, y0+h):
            for x in range(x0, x0+w):
                print('#' if (x, y) in self.occupied else '.', end='')
            print()
        print()

    def get_bounds(self) -> tuple[int, int, int, int]:
        min_x = min(x for (x, y) in self.elves)
        min_y = min(y for (x, y) in self.elves)
        max_x = max(x for (x, y) in self.elves)
        max_y = max(y for (x, y) in self.elves)
        return min_x, min_y, max_x - min_x + 1, max_y - min_y + 1

    def get_area(self) -> int:
        x, y, w, h = self.get_bounds()
        return w * h - len(self.elves)


def main() -> None:
    lines = sys.stdin.readlines()
    grove = Grove(lines)
    grove.simulate(10)
    result = grove.get_area()
    print(result)

    grove = Grove(lines)
    result2 = grove.simulate_until_stop()
    print(result2)


if __name__ == '__main__':
    main()
