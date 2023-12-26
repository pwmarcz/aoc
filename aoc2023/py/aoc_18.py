from dataclasses import dataclass
import re
import sys


@dataclass
class Command:
    dx: int
    dy: int
    n: int

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
        return cls(dx=dx, dy=dy, n=n)

    @classmethod
    def from_str2(cls, s: str) -> 'Command':
        m = re.match(r'^([UDLR]) (\d+) \(#([0-9a-f]{6})\)$', s)
        assert m, f"unrecognized: {s}"
        hex = m.group(3)
        n = int(hex[:5], 16)
        match hex[5]:
            case '3':
                dx, dy = (0, -1)
            case '1':
                dx, dy = (0, 1)
            case '2':
                dx, dy = (-1, 0)
            case '0':
                dx, dy = (1, 0)
            case _:
                assert False, f'unrecogized digit: {hex[5]}'
        return cls(dx=dx, dy=dy, n=n)


def make_map(commands: list[Command]) -> 'Map':
    x, y = 0, 0
    x_coord_set = {0}
    y_coord_set = {0}
    for cmd in commands:
        x += cmd.dx * cmd.n
        y += cmd.dy * cmd.n
        x_coord_set.add(x)
        x_coord_set.add(x+1)
        y_coord_set.add(y)
        y_coord_set.add(y+1)

    x_coords = sorted(x_coord_set)[:-1]
    y_coords = sorted(y_coord_set)[:-1]
    print(x_coords, y_coords)

    x, y = x_coords.index(x), y_coords.index(y)
    points = {(x, y)}
    for cmd in commands:
        x1 = x_coords.index(x_coords[x] + cmd.dx * cmd.n)
        y1 = y_coords.index(y_coords[y] + cmd.dy * cmd.n)
        if x1 == x:
            for yt in range(min(y, y1), max(y, y1) + 1):
                points.add((x, yt))
        else:
            assert y1 == y
            for xt in range(min(x, x1), max(x, x1) + 1):
                points.add((xt, y))
        x, y = x1, y1

    return Map(points, x_coords, y_coords)


class Map:
    def __init__(self, points: set[tuple[int, int]], x_coords: list[int], y_coords: list[int]) -> None:
        self.points = points
        self.x_coords = x_coords
        self.y_coords = y_coords

    def calc_area(self) -> int:
        stack: list[tuple[int, int]] = []
        for x in range(len(self.x_coords)):
            stack.append((x, 0))
            stack.append((x, len(self.y_coords) - 1))
        for y in range(len(self.y_coords)):
            stack.append((0, y))
            stack.append((len(self.x_coords) - 1, y))

        marked = self.flood_fill(stack)

        width = self.x_coords[-1] - self.x_coords[0] + 1
        height = self.y_coords[-1] - self.y_coords[0] + 1
        area = width * height
        # print(f'width {width}, height {height}')
        for x, y in marked:
            if x + 1 < len(self.x_coords):
                w = self.x_coords[x + 1] - self.x_coords[x]
            else:
                w = 1
            if y + 1 < len(self.y_coords):
                h = self.y_coords[y + 1] - self.y_coords[y]
            else:
                h = 1
            # print("point at {}, {} size {}".format(
            #     self.x_coords[x], self.y_coords[y], w*h
            # ))
            area -= w * h

        return area

    def flood_fill(self, stack: list[tuple[int, int]]) -> set[tuple[int, int]]:
        marked: set[tuple[int, int]] = set()
        while stack:
            x, y = stack.pop()
            if (x, y) in self.points or (x, y) in marked:
                continue
            marked.add((x, y))

            if x > 0:
                stack.append((x - 1, y))
            if x < len(self.x_coords) - 1:
                stack.append((x + 1, y))

            if y > 0:
                stack.append((x, y - 1))
            if y < len(self.y_coords) - 1:
                stack.append((x, y + 1))

        return marked


def main() -> None:
    lines = sys.stdin.read().splitlines()
    commands = [Command.from_str(line) for line in lines]
    map = make_map(commands)
    part1 = map.calc_area()

    commands = [Command.from_str2(line) for line in lines]
    map = make_map(commands)
    part2 = map.calc_area()

    print(part1, part2)


if __name__ == '__main__':
    main()
