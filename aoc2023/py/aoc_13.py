
import sys


class Pattern:
    def __init__(self, lines: list[str]) -> None:
        self.lines = lines

    def transpose(self) -> 'Pattern':
        lines_transposed = []
        for i in range(len(self.lines[0])):
            lines_transposed.append(
                ''.join(line[i] for line in self.lines)
            )
        return Pattern(lines_transposed)

    def find_reflective_row(self, smudges: int) -> int | None:
        for i in range(len(self.lines) - 1):
            if self.is_reflective_row(i, smudges):
                return i + 1
        return None

    def is_reflective_row(self, row: int, smudges: int) -> bool:
        for i in range(row + 1, len(self.lines)):
            j = row - (i - row) + 1
            if j >= 0:
                delta = sum(1 for a, b in zip(self.lines[i], self.lines[j]) if a != b)
                smudges -= delta
                if smudges < 0:
                    return False
        return smudges == 0


def main():
    parts = sys.stdin.read().split('\n\n')
    part1 = 0
    part2 = 0
    for part in parts:
        lines = part.splitlines()
        pattern = Pattern(lines)
        pattern_transposed = pattern.transpose()

        if (result := pattern_transposed.find_reflective_row(0)) is not None:
            part1 += result
        elif (result := pattern.find_reflective_row(0)) is not None:
            part1 += result * 100
        else:
            assert False, 'no reflection found'

        if (result := pattern_transposed.find_reflective_row(1)) is not None:
            part2 += result
        elif (result := pattern.find_reflective_row(1)) is not None:
            part2 += result * 100
        else:
            assert False, 'no reflection found'

    print(part1, part2)


if __name__ == '__main__':
    main()
