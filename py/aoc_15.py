
import sys
import re

from intervaltree import IntervalTree, Interval

READING_RE = re.compile(
    r'^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$'
)


def parse_reading(line):
    m = re.match(READING_RE, line)
    assert m, f'cannot parse: {line}'
    return (int(m.group(1)), int(m.group(2)), int(m.group(3)), int(m.group(4)))


def distance(x1, y1, x2, y2):
    return abs(x1 - x2) + abs(y1 - y2)


class RangeSet:
    def __init__(self):
        self.ranges = []


def get_tree_for_target(readings, target_y, remove_beacons=True):
    tree = IntervalTree()
    beacons = set()
    for sx, sy, bx, by in readings:
        dist_sensor_beacon = distance(sx, sy, bx, by)
        dist_sensor_target = abs(sy - target_y)
        dist_remaining = dist_sensor_beacon - dist_sensor_target
        if dist_remaining >= 0:
            start = sx - dist_remaining
            end = sx + dist_remaining + 1
            tree.addi(start, end)
        if by == target_y:
            beacons.add(bx)
    tree.merge_overlaps()
    if remove_beacons:
        for bx in beacons:
            tree.chop(bx, bx+1)
    # print(tree)
    return tree


def count_candidates(readings, target_y):
    tree = get_tree_for_target(readings, target_y)
    return sum(iv.end - iv.begin for iv in tree)


def find_spot(readings, y0, y1, h):
    tree_y = IntervalTree()
    tree_y.addi(0, h)
    for sx, sy, bx, by in readings:
        dist_sensor_beacon = distance(sx, sy, bx, by)
        margin_left = y0 - (sx - dist_sensor_beacon)
        margin_right = (sx + dist_sensor_beacon) - (y1 + 1)
        margin = min(margin_left, margin_right)
        #print(margin_left, margin_right)
        if margin >= 0:
            tree_y.chop(sy - margin, sy + margin + 1)
    print('tree_y', tree_y)

    for iv_y in tree_y:
        for y in range(iv_y.begin, iv_y.end):
            tree = IntervalTree()
            tree.addi(y0, y1 + 1)
            for iv in get_tree_for_target(readings, y, remove_beacons=False):
                tree.chop(iv.begin, iv.end)
            if len(tree) > 0:
                print(tree)
                x = sorted(tree)[0].begin
                return x, y, x * 4000000 + y


def main():
    lines = list(sys.stdin)
    readings = [parse_reading(line.rstrip()) for line in lines]

    print('count', count_candidates(readings, 10))
    print('count', count_candidates(readings, 2_000_000))
    print('spot', find_spot(readings, 0, 20, 20))
    step = 50_000
    for x0 in range(0, 4_000_000, step):
        x1 = x0 + step
        print('strip', x0, x1)
        spot = find_spot(readings, x0, x1, 4000000)
        if spot:
            print('spot', spot)
            break




if __name__ == '__main__':
    main()
