import sys


def sign(x):
    if x > 0:
        return 1
    elif x < 0:
        return -1
    else:
        return 0


def update_rope(rope, seen, dx, dy):
    x0, y0 = rope[0]
    x0 += dx
    y0 += dy
    rope[0] = x0, y0
    for i in range(1, len(rope)):
        x, y = rope[i]
        if abs(x0 - x) > 1 or abs(y0 - y) > 1:
            x += sign(x0 - x)
            y += sign(y0 - y)
        rope[i] = x, y
        x0, y0 = x, y

    seen.add((x0, y0))


def main():
    start = (0, 0)
    rope1, seen1 = [start] * 2, {start}
    rope2, seen2 = [start] * 10, {start}

    for line in sys.stdin.read().splitlines():
        op, _space, n = line.rstrip().partition(' ')
        n = int(n)
        dx, dy = {
            'U': (0, -1),
            'D': (0, 1),
            'L': (-1, 0),
            'R': (1, 0),
        }[op]

        for i in range(n):
            update_rope(rope1, seen1, dx, dy)
            update_rope(rope2, seen2, dx, dy)

    print(len(seen1), len(seen2))


if __name__ == '__main__':
    main()
