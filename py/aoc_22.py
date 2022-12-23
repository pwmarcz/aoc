from enum import Enum
import re
import sys


class Dir(Enum):
    R = (1, 0)
    D = (0, 1)
    L = (-1, 0)
    U = (0, -1)

    @property
    def score(self) -> int:
        return {
            Dir.R: 0,
            Dir.D: 1,
            Dir.L: 2,
            Dir.U: 3
        }[self]

    @property
    def char(self) -> str:
        return {
            Dir.R: '>',
            Dir.D: 'v',
            Dir.L: '<',
            Dir.U: '^'
        }[self]

    @property
    def cw(self) -> 'Dir':
        return {
            Dir.U: Dir.R,
            Dir.R: Dir.D,
            Dir.D: Dir.L,
            Dir.L: Dir.U
        }[self]

    @property
    def ccw(self) -> 'Dir':
        return {
            Dir.U: Dir.L,
            Dir.L: Dir.D,
            Dir.D: Dir.R,
            Dir.R: Dir.U
        }[self]

    @property
    def opposite(self) -> 'Dir':
        return {
            Dir.U: Dir.D,
            Dir.L: Dir.R,
            Dir.D: Dir.U,
            Dir.R: Dir.L,
        }[self]


class Map:
    def __init__(self, lines: list[str]):
        self.lines = lines
        self.rows: list[tuple[int, int]] = []
        self.cols: list[tuple[int, int]] = []
        for line in self.lines:
            start, end = 9999, 0
            for x, c in enumerate(line):
                if c != ' ':
                    start = min(start, x)
                    end = max(end, x + 1)
            self.rows.append((start, end))
        width = max(end for start, end in self.rows)
        for x in range(width):
            start, end = 9999, 0
            for y, line in enumerate(self.lines):
                c = line[x] if x < len(line) else ' '
                if c != ' ':
                    start = min(start, y)
                    end = max(end, y + 1)
            self.cols.append((start, end))

    def run(self, sequence: str, cube: bool = False, draw: bool = False) -> tuple[int, int, Dir]:
        x, y = self.rows[0][0], 0
        d = Dir.R
        for part in re.findall(r'([LR]|\d+)', sequence):
            if part == 'R':
                d = d.cw
            elif part == 'L':
                d = d.ccw
            else:
                steps = int(part)
                for _ in range(steps):
                    if cube:
                        x, y, d = self.cube_forward(x, y, d)
                    else:
                        x, y = self.forward(x, y, d)
                    if draw:
                        self.lines[y] = self.lines[y][:x] + d.char + self.lines[y][x+1:]
                        # print('\n'.join(self.lines))
                        # print()

        return x, y, d

    def forward(self, x: int, y: int, d: Dir) -> tuple[int, int]:
        x1 = x + d.value[0]
        y1 = y + d.value[1]
        if x1 < self.rows[y][0]:
            x1 = self.rows[y][1] - 1
        elif x1 >= self.rows[y][1]:
            x1 = self.rows[y][0]
        if y1 < self.cols[x][0]:
            y1 = self.cols[x][1] - 1
        elif y1 >= self.cols[x][1]:
            y1 = self.cols[x][0]
        c = self.lines[y1][x1]
        assert c != ' '
        if c == '#':
            return x, y
        return x1, y1

    def cube_forward(self, x: int, y: int, d: Dir) -> tuple[int, int, Dir]:
        x1 = x + d.value[0]
        y1 = y + d.value[1]
        d1 = d
        if not (self.rows[y][0] <= x1 < self.rows[y][1] and
                self.cols[x][0] <= y1 < self.cols[x][1]):
            if len(self.lines[0]) < 50:
                n = 4
                cube = CUBE_SMALL
            else:
                n = 50
                cube = CUBE_BIG

            xx, yy = x // n, y // n
            match d:
                case Dir.U:
                    k = x % n
                case Dir.D:
                    k = n - x % n - 1
                case Dir.L:
                    k = n - y % n - 1
                case Dir.R:
                    k = y % n

            xx1, yy1, d1 = cube[(xx, yy)][d]
            match d1:
                case Dir.D:
                    x1 = (xx1 + 1) * n - k - 1
                    y1 = yy1 * n
                case Dir.U:
                    x1 = xx1 * n + k
                    y1 = (yy1 + 1) * n - 1
                case Dir.L:
                    x1 = (xx1 + 1) * n - 1
                    y1 = (yy1 + 1) * n - k - 1
                case Dir.R:
                    x1 = xx1 * n
                    y1 = yy1 * n + k

        c = self.lines[y1][x1]
        assert c != ' '
        if c == '#':
            return x, y, d
        return x1, y1, d1


CUBE_SMALL: dict[tuple[int, int], dict[Dir, tuple[int, int, Dir]]] = {
    (2, 0): {
        Dir.U: (0, 1, Dir.D),
        Dir.L: (1, 1, Dir.D),
        Dir.R: (3, 2, Dir.L),
    },
    (0, 1): {
        Dir.U: (2, 0, Dir.D),
        Dir.L: (3, 2, Dir.U),
        Dir.D: (2, 2, Dir.U),
    },
    (1, 1): {
        Dir.U: (2, 0, Dir.R),
        Dir.D: (2, 2, Dir.R),
    },
    (2, 1): {
        Dir.R: (3, 2, Dir.D),
    },
    (2, 2): {
        Dir.L: (1, 1, Dir.U),
        Dir.D: (0, 1, Dir.U),
    },
    (3, 2): {
        Dir.U: (2, 1, Dir.L),
        Dir.R: (2, 0, Dir.L),
        Dir.D: (0, 1, Dir.R),
    }
}


CUBE_BIG: dict[tuple[int, int], dict[Dir, tuple[int, int, Dir]]] = {
    (1, 0): {
        Dir.U: (0, 3, Dir.R),
        Dir.L: (0, 2, Dir.R),
    },
    (2, 0): {
        Dir.U: (0, 3, Dir.U),
        Dir.R: (1, 2, Dir.L),
        Dir.D: (1, 1, Dir.L),
    },
    (1, 1): {
        Dir.L: (0, 2, Dir.D),
        Dir.R: (2, 0, Dir.U),
    },
    (0, 2): {
        Dir.U: (1, 1, Dir.R),
        Dir.L: (1, 0, Dir.R),
    },
    (1, 2): {
        Dir.R: (2, 0, Dir.L),
        Dir.D: (0, 3, Dir.L),
    },
    (0, 3): {
        Dir.L: (1, 0, Dir.D),
        Dir.D: (2, 0, Dir.D),
        Dir.R: (1, 2, Dir.U),
    }
}


def check(cube):
    for (xx, yy), edges in cube.items():
        for d, (xx1, yy1, d1) in edges.items():
            assert cube[(xx1, yy1)][d1.opposite] == (xx, yy, d.opposite), (xx, yy, d)


check(CUBE_SMALL)
check(CUBE_BIG)


def main():
    map_s, _, sequence = sys.stdin.read().partition('\n\n')
    map = Map(map_s.splitlines())
    x, y, d = map.run(sequence)
    result = (y + 1) * 1000 + (x + 1) * 4 + d.score
    print(result)

    x, y, d = map.run(sequence, cube=True, draw=True)
    print('\n'.join(map.lines))
    result = (y + 1) * 1000 + (x + 1) * 4 + d.score
    print(result)


if __name__ == '__main__':
    main()
