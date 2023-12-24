import functools
import sys

Box = list[tuple[str, int]]


def lava_hash(s: str) -> int:
    return functools.reduce(lambda h, c: (h + ord(c)) * 17 % 256, s, 0)


def remove_lens(box: Box, label: str) -> None:
    for i, lens in enumerate(box):
        if lens[0] == label:
            box.pop(i)
            return


def replace_or_add_lens(box: Box, label: str, focus: int) -> None:
    for i, lens in enumerate(box):
        if lens[0] == label:
            box[i] = (label, focus)
            return
    box.append((label, focus))


def apply_step(boxes: dict[int, Box], step: str) -> None:
    if step.endswith('-'):
        label = step[:-1]
        box_no = lava_hash(label)
        box = boxes.setdefault(box_no, [])
        remove_lens(box, label)
    else:
        assert step.count('=') == 1
        label, focus_str = step.split('=')
        focus = int(focus_str)
        box_no = lava_hash(label)
        box = boxes.setdefault(box_no, [])
        replace_or_add_lens(box, label, focus)


def calculate_focus_power(boxes: dict[int, Box]):
    total = 0
    for num, box in boxes.items():
        for i, (_label, focus) in enumerate(box):
            total += (num + 1) * (i + 1) * focus
    return total


def main() -> None:
    input = sys.stdin.read().rstrip()
    steps = input.split(',')
    part1 = sum(lava_hash(step) for step in steps)

    boxes: dict[int, Box] = {}
    for step in steps:
        apply_step(boxes, step)

    part2 = calculate_focus_power(boxes)
    print(part1, part2)


if __name__ == '__main__':
    main()
