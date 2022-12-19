import re
import sys


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
        self.states = [[] for _ in range(self.n)]

    def run(
        self,
        minute=0,
        robots=[1, 0, 0, 0],
        resources=[0, 0, 0, 0],
        history='',
    ):
        if minute == self.n:
            if resources[3] > self.best:
                self.best = resources[3]
                print(self.best, history)
            return self.best

        remaining = self.n - minute
        estimated = resources[3] + remaining * robots[3]
        if any(r + rr < c for r, rr, c in zip(resources, robots, self.blueprint[3])):
            estimated += max(0, remaining - 2) * max(0, remaining - 3) // 2
        if any(r < c for r, c in zip(resources, self.blueprint[3])):
            estimated += max(0, remaining - 1) * max(0, remaining - 2) // 2
        else:
            estimated += remaining * max(0, remaining - 1) // 2
        if estimated <= self.best:
            return self.best
        # print(f"{minute:2} {history:25} {self.best:2} {str(robots):10} {str(resources):20}")

        state = robots + resources
        for old_state in self.states[minute]:
            if all(x <= y for x, y in zip(state, old_state)):
                return
        self.states[minute].append(state)

        for i in [3, 2, 1, 0]:
            costs = self.blueprint[i]
            if all(r >= c for r, c in zip(resources, costs)):
                new_resources = [r-c+rr for r, c, rr in zip(resources, costs, robots)]
                new_robots = robots.copy()
                new_robots[i] += 1
                self.run(minute+1, new_robots, new_resources, history=history+str(i))

        new_resources = [r+rr for r, rr in zip(resources, robots)]
        self.run(minute+1, robots, new_resources, history=history+'.')

        # self.states[minute] = state
        return self.best


def run_test(blueprints):
    total = 0
    for i, b in enumerate(blueprints):
        best = Backtrack(b, 24).run()
        total += best
        print(i+1, '--->', best)
    return total


def main():
    print(run_test(example_blueprints))
    return
    lines = sys.stdin.readlines()
    blueprints = [parse_blueprint(line) for line in lines]
    print(run_test(blueprints))


if __name__ == "__main__":
    main()
