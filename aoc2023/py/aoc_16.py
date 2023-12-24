import sys


class Map:
    rows: list[list[str]]

    def __init__(self, s: str) -> None:
        self.rows = [list(line) for line in s.splitlines()]
        self.next = {}

    @property
    def w(self):
        return len(self.rows[0])

    @property
    def h(self):
        return len(self.rows)

    def advance(self, x: int, y: int, dx: int, dy: int) -> list[tuple[int, int]]:
        match self.rows[y][x]:
            case '.':
                return [(dx, dy)]
            case '/':
                return [(-dy, -dx)]
            case '\\':
                return [(dy, dx)]
            case '|':
                if dx == 0:
                    return [(dx, dy)]
                else:
                    return [(0, -1), (0, 1)]
            case '-':
                if dy == 0:
                    return [(dx, dy)]
                else:
                    return [(-1, 0), (1, 0)]
            case c:
                assert False, f"unknown: {c}"

    def in_bounds(self, x: int, y: int) -> bool:
        return 0 <= x < self.w and 0 <= y < self.h

    def run(self) -> int:
        return self.run_from(0, 0, 1, 0)

    def run_any(self) -> int:
        best = 0
        for x in range(self.w):
            cur = self.run_from(x, 0, 0, 1)
            best = max(best, cur)
            cur = self.run_from(x, self.h-1, 0, -1)
            best = max(best, cur)
            print(f'{x + 1} / {self.w + self.h}')

        for y in range(self.h):
            cur = self.run_from(0, y, 1, 0)
            best = max(best, cur)
            cur = self.run_from(self.w-1, y, -1, 0)
            best = max(best, cur)
            print(f'{self.w + y + 1} / {self.w + self.h}')

        return best

    def run_from(self, x: int, y: int, dx: int, dy: int) -> int:
        rays = {(x, y, dx, dy)}
        all_rays = rays.copy()
        while True:
            n = len(all_rays)

            new_rays = set()
            for x0, y0, dx0, dy0 in rays:
                for dx, dy in self.advance(x0, y0, dx0, dy0):
                    x = x0 + dx
                    y = y0 + dy
                    if self.in_bounds(x, y):
                        ray = (x, y, dx, dy)
                        if ray not in all_rays:
                            new_rays.add(ray)
                            all_rays.add(ray)
            rays = new_rays
            if len(all_rays) == n:
                break

        energized = set((x, y) for (x, y, _dx, _dy) in all_rays)
        return len(energized)

    def __str__(self):
        return '\n'.join(''.join(row) for row in self.rows)


def main() -> None:
    input = sys.stdin.read()
    map = Map(input)
    part1 = map.run()
    part2 = map.run_any()
    print(part1, part2)


if __name__ == '__main__':
    main()
