import ast
import sys


NEIGHBORS = [
    (1, 0, 0), (-1, 0, 0),
    (0, 1, 0), (0, -1, 0),
    (0, 0, 1), (0, 0, -1),
]


def surface_area(cubes):
    area = 0
    for x, y, z in cubes:
        for dx, dy, dz in NEIGHBORS:
            if (x+dx, y+dy, z+dz) not in cubes:
                area += 1
    return area


def external_area(cubes):
    x0 = min(x for (x, y, z) in cubes) - 1
    x1 = max(x for (x, y, z) in cubes) + 1
    y0 = min(y for (x, y, z) in cubes) - 1
    y1 = max(y for (x, y, z) in cubes) + 1
    z0 = min(z for (x, y, z) in cubes) - 1
    z1 = max(z for (x, y, z) in cubes) + 1

    seen = set()
    stack = [(x0, y0, z0)]
    area = 0

    while len(stack) > 0:
        p = x, y, z = stack.pop()
        if p in seen or not ((x0 <= x <= x1) and (y0 <= y <= y1) and (z0 <= z <= z1)):
            continue
        seen.add(p)
        for dx, dy, dz in NEIGHBORS:
            neighbor = (x+dx, y+dy, z+dz)
            if neighbor in cubes:
                area += 1
            else:
                stack.append(neighbor)

    return area


def main():
    lines = sys.stdin.readlines()
    cubes = set(ast.literal_eval(line) for line in lines)
    print(surface_area(cubes))
    print(external_area(cubes))


if __name__ == '__main__':
    main()
