import sys
from dataclasses import dataclass
import heapq
from collections.abc import Iterator


@dataclass(frozen=True, order=True)
class State:
    x: int
    y: int
    dx: int
    dy: int
    turns: int


class Map:
    rows: list[list[int]]

    def __init__(self, s: str) -> None:
        self.rows = [[int(c) for c in line] for line in s.splitlines()]

    @property
    def w(self) -> int:
        return len(self.rows[0])

    @property
    def h(self) -> int:
        return len(self.rows)

    def in_bounds(self, x: int, y: int) -> bool:
        return 0 <= x < self.w and 0 <= y < self.h

    def search(self, ultra: bool = False) -> int:
        start = State(x=0, y=0, dx=0, dy=0, turns=0)
        goal = (self.w - 1, self.h - 1)

        queue: list[tuple[int, State]] = []
        heapq.heappush(queue, (0, start))
        seen: set[State] = {start}
        while queue:
            cost, state = heapq.heappop(queue)
            if (state.x, state.y) == goal and (not ultra or state.turns >= 4):
                return cost
            for next_cost, next_state in self.advance(cost, state, ultra=ultra):
                if next_state not in seen:
                    heapq.heappush(queue, (next_cost, next_state))
                    seen.add(next_state)

        assert False, 'no path found'

    def advance(self, cost: int, state: State, ultra: bool) -> Iterator[tuple[int, State]]:
        for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            x = state.x + dx
            y = state.y + dy
            if not self.in_bounds(x, y):
                continue
            if (dx, dy) == (state.dx, state.dy) or (state.dx, state.dy) == (0, 0):
                if state.turns == (10 if ultra else 3):
                    continue
                turns = state.turns + 1
            elif (dx, dy) == (-state.dx, state.dy) or (dx, dy) == (state.dx, -state.dy):
                continue
            else:
                if ultra and state.turns < 4:
                    continue
                turns = 1
            next_cost = cost + self.rows[y][x]
            yield next_cost, State(x=x, y=y, dx=dx, dy=dy, turns=turns)


def main() -> None:
    input = sys.stdin.read()
    map = Map(input)
    part1 = map.search()
    part2 = map.search(ultra=True)
    print(part1, part2)


if __name__ == '__main__':
    main()
