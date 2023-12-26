from dataclasses import dataclass
import re
import sys


@dataclass
class Command:
    dx: int
    dy: int
    n: int
    hex: str

    @classmethod
    def from_str(cls, s: str) -> 'Command':
        m = re.match(r'^([UDLR]) (\d+) \(#([0-9a-f]{6})\)$', s)
        assert m, f"unrecognized: {s}"
        match m.group(1):
            case 'U':
                dx, dy = (0, -1)
            case 'D':
                dx, dy = (0, 1)
            case 'L':
                dx, dy = (-1, 0)
            case 'R':
                dx, dy = (1, 0)
            case _:
                assert False
        n = int(m.group(2))
        hex = m.group(3)
        return cls(dx=dx, dy=dy, n=n, hex=hex)


def make_map(commands: list[Command]) -> 'Map':
    x, y = 0, 0
    points = {(x, y)}
    for cmd in commands:
        for i in range(cmd.n):
            x += cmd.dx
            y += cmd.dy
            points.add((x, y))
    return Map(points)


class Map:
    def __init__(self, points: set[tuple[int, int]]) -> None:
        self.points = points
        min_x, min_y = 0, 0
        max_x, max_y = 0, 0
        for x, y in points:
            min_x = min(min_x, x)
            min_y = min(min_y, y)
            max_x = max(max_x, x)
            max_y = max(max_y, y)
        self.min_x = min_x
        self.min_y = min_y
        self.max_x = max_x
        self.max_y = max_y

    def calc_area(self) -> int:
        stack: list[tuple[int, int]] = []
        for x in range(self.min_x, self.max_x + 1):
            stack.append((x, self.min_y))
            stack.append((x, self.max_y))
        for y in range(self.min_y, self.max_y + 1):
            stack.append((self.min_x, y))
            stack.append((self.max_x, y))

        marked = self.flood_fill(stack)

        width = self.max_x - self.min_x + 1
        height = self.max_y - self.min_y + 1
        return width * height - len(marked)

    def flood_fill(self, stack: list[tuple[int, int]]) -> set[tuple[int, int]]:
        marked: set[tuple[int, int]] = set()
        while stack:
            x, y = stack.pop()
            if (x, y) in self.points or (x, y) in marked:
                continue
            marked.add((x, y))
            if x > self.min_x:
                stack.append((x - 1, y))
            if x < self.max_x:
                stack.append((x + 1, y))
            if y > self.min_y:
                stack.append((x, y - 1))
            if y < self.max_y:
                stack.append((x, y + 1))
        return marked


def main() -> None:
    lines = sys.stdin.read().splitlines()
    commands = [Command.from_str(line) for line in lines]
    map = make_map(commands)
    part1 = map.calc_area()
    print(part1)


if __name__ == '__main__':
    main()
