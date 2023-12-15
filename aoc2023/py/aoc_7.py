from collections import Counter
import sys


CARDS_PART1 = '23456789TJQKA'
CARDS_PART2 = 'J23456789TQKA'


def parse_line(line: str):
    hand, score_str = line.split(' ', 1)
    score = int(score_str)
    return hand, score


def get_type(hand: str):
    c = Counter(hand)
    m = c.most_common()
    first_count = m[0][1]
    second_count = m[1][1] if len(m) > 1 else None
    match first_count, second_count:
        case 5, _:
            return 6
        case 4, _:
            return 5
        case 3, 2:
            return 4
        case 3, _:
            return 3
        case 2, 2:
            return 2
        case 2, _:
            return 1
        case _:
            return 0


def score_hand_part1(hand: str):
    t = get_type(hand)
    converted = [CARDS_PART1.index(c) for c in hand]
    # print(f'{hand} has type {t} and is converted to {converted}')
    return t, converted


def score_hand_part2(hand: str):
    t = get_type(hand)
    for c in CARDS_PART2[1:]:
        t = max(t, get_type(hand.replace('J', c)))
    converted = [CARDS_PART2.index(c) for c in hand]
    # print(f'{hand} has type {t} and is converted to {converted}')
    return t, converted


def main():
    hands = [parse_line(line) for line in sys.stdin.readlines()]

    hands.sort(key=lambda x: score_hand_part1(x[0]))
    part1 = 0
    for rank, (hand, bid) in enumerate(hands, 1):
        # print(rank, hand)
        part1 += rank * bid
    print(part1)

    hands.sort(key=lambda x: score_hand_part2(x[0]))
    part2 = 0
    for rank, (hand, bid) in enumerate(hands, 1):
        # print(rank, hand)
        part2 += rank * bid
    print(part2)


if __name__ == '__main__':
    main()
