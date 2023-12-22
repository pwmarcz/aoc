
import sys


MEMO = {}


def backtrack(
    row: str,
    numbers: tuple[int, ...],
    cur_number: int | None = None
) -> int:
    if len(row) == 0:
        if len(numbers) == 0 and (cur_number == 0 or cur_number is None):
            return 1
        return 0

    key = (row, numbers, cur_number)
    if key in MEMO:
        return MEMO[key]

    result = 0

    if row[0] in '?#' and cur_number is None and len(numbers) > 0:
        # try '#' and start new chunk
        result += backtrack(row[1:], numbers[1:], numbers[0] - 1)

    if row[0] in '?#' and cur_number is not None and cur_number > 0:
        # try '#' and continue current chunk
        result += backtrack(row[1:], numbers, cur_number - 1)

    if row[0] in '?.' and (cur_number is None or cur_number == 0):
        # try '.'
        result += backtrack(row[1:], numbers, None)

    MEMO[key] = result

    return result


def main() -> None:
    part1 = 0
    part2 = 0
    for line in sys.stdin.readlines():
        row, numbers_str = line.split(' ', 1)
        numbers = tuple(int(s) for s in numbers_str.split(','))
        result = backtrack(row, numbers)
        part1 += result

        result = backtrack('?'.join([row] * 5), numbers * 5)
        print(result)
        part2 += result

    print(part1, part2)

if __name__ == '__main__':
    main()
