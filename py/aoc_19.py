import re
import sys
from collections import deque


example_blueprints = [
    ((4, 0, 0, 0), (2, 0, 0, 0), (3, 14, 0, 0), (2, 0, 7, 0)),
    ((2, 0, 0, 0), (3, 0, 0, 0), (3, 8, 0, 0), (3, 0, 12, 0)),
]


BLUEPRINT_RE = re.compile(
    r"^Blueprint \d+: Each ore robot costs (\d+) ore. "
    r"Each clay robot costs (\d+) ore. "
    r"Each obsidian robot costs (\d+) ore and (\d+) clay. "
    r"Each geode robot costs (\d+) ore and (\d+) obsidian.$"
)


def parse_blueprint(s):
    m = BLUEPRINT_RE.match(s)
    assert m
    return (
        (int(m.group(1)), 0, 0, 0),
        (int(m.group(2)), 0, 0, 0),
        (int(m.group(3)), int(m.group(4)), 0, 0),
        (int(m.group(5)), 0, int(m.group(6)), 0),
    )


class Backtrack:
    def __init__(self, blueprint, n):
        self.blueprint = blueprint
        self.best = 0
        self.n = n
        self.states = [deque() for _ in range(self.n)]
        self.robots = [1, 0, 0, 0]
        self.resources = [0, 0, 0, 0]
        self.history = []

    def run(
        self,
        minute=0,
    ):
        if minute == self.n:
            if self.resources[3] > self.best:
                self.best = self.resources[3]
                print(self.best, ''.join(self.history))
            return

        remaining = self.n - minute
        estimated = self.resources[3] + remaining * self.robots[3]
        if any(c > r + rr for c, r, rr in zip(self.blueprint[3], self.resources, self.robots)):
            estimated += max(0, remaining - 2) * max(0, remaining - 3) // 2
        if any(c > r for c, r in zip(self.blueprint[3], self.resources)):
            estimated += max(0, remaining - 1) * max(0, remaining - 2) // 2
        else:
            estimated += remaining * max(0, remaining - 1) // 2
        if estimated <= self.best:
            return

        state = self.robots + self.resources
        queue = self.states[minute]
        for old_state in queue:
            if all(x <= y for x, y in zip(state, old_state)):
                return
        queue.append(state)
        if len(queue) > 10:
            queue.popleft()

        order = [i for (_, _,  i) in sorted([
            (0, 0, 3),
            (self.resources[2] / self.blueprint[3][2], 1, 2),
            (self.resources[1] / self.blueprint[2][1], 2, 1),
            (self.resources[0] / self.blueprint[2][0], 3, 0),
        ])]
        print(minute, self.best, ''.join(self.history), order)

        for i in order:
            costs = self.blueprint[i]
            if all(r >= c for r, c in zip(self.resources, costs)):
                for j in range(4):
                    self.resources[j] += self.robots[j] - costs[j]
                self.robots[i] += 1
                self.history.append(str(i))
                self.run(minute+1)
                self.history.pop()
                self.robots[i] -= 1
                for j in range(4):
                    self.resources[j] -= self.robots[j] - costs[j]

                if i == 3:
                    return

        for j in range(4):
            self.resources[j] += self.robots[j]
        self.history.append('.')
        self.run(minute+1)
        self.history.pop()
        for j in range(4):
            self.resources[j] -= self.robots[j]

        # self.states[minute] = state
        return


def run_test(blueprints):
    total = 0
    for i, b in enumerate(blueprints):
        bt = Backtrack(b, 24)
        bt.run()
        total += (i+1) * bt.best
        print(i+1, '--->', bt.best)
    return total


def run_test2(blueprints):
    total = 1
    for i, b in enumerate(blueprints):
        bt = Backtrack(b, 32)
        bt.run()
        total *= bt.best
        print(i+1, '--->', bt.best)
    return total


def main(example=False, idx=None):
    if example:
        blueprints = example_blueprints
    else:
        blueprints = [parse_blueprint(line) for line in sys.stdin.readlines()]
    if idx is not None:
        blueprints = [blueprints[idx]]
    print(run_test2(blueprints[:3]))


if __name__ == "__main__":
    example = False
    idx = None
    for arg in sys.argv[1:]:
        if arg == 'example':
            example = True
        else:
            idx = int(arg)
    main(example, idx)
