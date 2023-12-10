
import re
import sys


def process(lines: list[str]) -> tuple[int, int]:
    symbols = dict()
    for y, line in enumerate(lines):
        for m in re.finditer(r'[^0-9.\n]', line):
            x = m.start()
            symbols[(x, y)] = m.group(0)

    numbers = []
    numbers_by_star = {}
    for y, line in enumerate(lines):
        for m in re.finditer(r'[0-9]+', line):
            num = int(m.group())
            x1 = m.start()
            x2 = m.end()
            search_coords = {(x1 - 1, y), (x2, y)}
            for x in range(x1 - 1, x2 + 1):
                search_coords.add((x, y - 1))
                search_coords.add((x, y + 1))
            for k in search_coords:
                if v := symbols.get(k):
                    numbers.append(num)
                    if v == '*':
                        numbers_by_star.setdefault(k, []).append(num)

    part1 = sum(numbers)

    part2 = 0
    for ns in numbers_by_star.values():
        if len(ns) == 2:
            part2 += ns[0] * ns[1]

    return part1, part2
                

def main() -> None:
    print(process(sys.stdin.readlines()))

if __name__ == '__main__':
    main()