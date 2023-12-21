import itertools
import math
import re
import sys


def parse_line(line: str):
    m = re.match(r'^(\w{3}) = \((\w{3}), (\w{3})\)', line)
    assert m, repr(line)
    return m.group(1), m.group(2), m.group(3)


def parse_all(lines: list[str]):
    path = lines[0].rstrip()
    nodes = {}
    for line in lines[2:]:
        src, left, right = parse_line(line.rstrip())
        nodes[src] = left, right
    return path, nodes


def part1(path: str, nodes: dict[str, tuple[str, str]]) -> int:
    n = 0
    node = 'AAA'
    for c in itertools.cycle(path):
        left, right = nodes[node]
        node = left if c == 'L' else right
        n += 1
        if node == 'ZZZ':
            return n

    assert False


def find_cycle(path: str, nodes: dict[str, tuple[str, str]], start: str):
    node = start
    seen = {(0, start): 0}
    history = [start]
    for c in itertools.cycle(path):
        left, right = nodes[node]
        node = left if c == 'L' else right
        n = len(history)
        if (prev := seen.get((n % len(path), node))):
            end_nodes = [i for i, node in enumerate(history) if node.endswith('Z')]
            return n - prev, end_nodes

        seen[(n % len(path), node)] = n
        history.append(node)

    assert False


def part2(path: str, nodes: dict[str, tuple[str, str]]) -> int:
    cycles = []
    for node in nodes:
        if node.endswith('A'):
            cycle, end_nodes = find_cycle(path, nodes, node)
            assert len(end_nodes) == 1
            assert cycle == end_nodes[0]
            cycles.append(cycle)

    return math.lcm(*cycles)

def main():
    path, nodes = parse_all(sys.stdin.readlines())
    print(part1(path, nodes))
    print(part2(path, nodes))
    nodes = {node for node in nodes if node.endswith('Z')}


if __name__ == '__main__':
    main()
