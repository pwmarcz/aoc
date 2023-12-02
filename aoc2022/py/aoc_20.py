import sys
from dataclasses import dataclass


@dataclass
class Node:
    val: int
    pos: int


def mix(arr, times=1):
    nodes = [Node(val, pos) for pos, val in enumerate(arr)]
    nodes_orig = nodes.copy()
    # print([node.val for node in nodes])
    for k in range(times):
        print(f'mix {k}')
        for node in nodes_orig:
            val = node.val
            pos = node.pos
            new_pos = (pos + val) % (len(arr) - 1)
            if val != 0 and new_pos != pos:
                if pos < new_pos:
                    for i in range(pos, new_pos):
                        nodes[i] = nodes[i+1]
                        nodes[i].pos = i
                else:
                    for i in range(pos, new_pos, -1):
                        nodes[i] = nodes[i-1]
                        nodes[i].pos = i
                nodes[new_pos] = node
                nodes[new_pos].pos = new_pos
        # print([node.val for node in nodes])
    return [node.val for node in nodes]


def coords(arr):
    zero_idx = arr.index(0)
    a = arr[(zero_idx + 1000) % len(arr)]
    b = arr[(zero_idx + 2000) % len(arr)]
    c = arr[(zero_idx + 3000) % len(arr)]
    result = a + b + c
    return result


def main():
    arr = [int(line) for line in sys.stdin.readlines()]

    arr1 = mix(arr)
    print(arr1)
    print(coords(arr1))

    arr2 = mix([x * 811589153 for x in arr], 10)
    print(arr2)
    print(coords(arr2))


if __name__ == '__main__':
    main()
