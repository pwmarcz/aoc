import builtins
import sys
import ast
import functools


def compare(a: list | int, b: list | int) -> int:
    match type(a), type(b):
        case builtins.int, builtins.int:
            if a < b:
                return -1
            elif a > b:
                return 1
            else:
                return 0
        case builtins.list, builtins.list:
            for ax, bx in zip(a, b):
                c = compare(ax, bx)
                if c != 0:
                    return c
            return compare(len(a), len(b))
        case builtins.int, builtins.list:
            return compare([a], b)
        case builtins.list, builtins.int:
            return compare(a, [b])


def part1(s):
    result = 0
    pairs = s.split('\n\n')
    for i, p in enumerate(pairs):
        pa, pb = p.strip().split('\n')
        a = ast.literal_eval(pa)
        b = ast.literal_eval(pb)
        c = compare(a, b)
        assert c != 0
        if c == -1:
            result += i + 1
    return result


def part2(s):
    items = [ast.literal_eval(line) for line in s.splitlines() if line.strip() != '']
    items.append([[2]])
    items.append([[6]])
    items.sort(key=functools.cmp_to_key(compare))
    a = items.index([[2]])
    b = items.index([[6]])
    assert a >= 0
    assert b >= 0
    return (a + 1) * (b + 1)


def main():
    s = sys.stdin.read()
    print(part1(s))
    print(part2(s))


if __name__ == '__main__':
    main()
