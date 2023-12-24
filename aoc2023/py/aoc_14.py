import sys


class Map:
    rows: list[list[str]]

    def __init__(self, s: str) -> None:
        self.update(s)

    def update(self, s: str):
        self.rows = [list(line) for line in s.splitlines()]

    @property
    def w(self):
        return len(self.rows[0])

    @property
    def h(self):
        return len(self.rows)

    def spin(self, n: int) -> None:
        results: list[str] = []
        numbers: dict[str, int] = {}
        for i in range(1, n):
            s = str(self)

            if j := numbers.get(s):
                print(f'Found cycle from {j} to {i}')
                remainder = j + (n - j) % (i - j)
                self.update(results[remainder])
                return

            results.append(s)
            numbers[s] = i

            self.roll_ns(True)
            self.roll_we(True)
            self.roll_ns(False)
            self.roll_we(False)

    def roll_ns(self, is_north: bool) -> None:
        for x in range(self.w):
            column = [self.rows[y][x] for y in range(self.h)]
            self.roll_column(column, reverse=is_north)
            for y in range(self.h):
                self.rows[y][x] = column[y]

    def roll_we(self, is_west: bool) -> None:
        for y in range(self.h):
            column = list(self.rows[y])
            self.roll_column(column, reverse=is_west)
            self.rows[y] = column

    def roll_column(self, column: list[str], reverse: bool) -> None:
        span = 0
        if reverse:
            column.reverse()
        column.append('#')
        for i in range(len(column)):
            match column[i]:
                case '.':
                    pass
                case 'O':
                    column[i] = '.'
                    span += 1
                case '#':
                    for j in range(i - span, i):
                        column[j] = 'O'
                    span = 0
        column.pop()
        if reverse:
            column.reverse()

    def get_load(self) -> int:
        result = 0
        for y in range(self.h):
            for x in range(self.w):
                if self.rows[y][x] == 'O':
                    result += self.h - y
        return result

    def __str__(self):
        return '\n'.join(''.join(row) for row in self.rows)


def main() -> None:
    input = sys.stdin.read()
    map = Map(input)
    map.roll_ns(True)
    part1 = map.get_load()

    map = Map(input)
    map.spin(1_000_000_000)
    part2 = map.get_load()
    print(part1, part2)


if __name__ == '__main__':
    main()
