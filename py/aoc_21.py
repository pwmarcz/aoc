
from dataclasses import dataclass
import re
import sys
from typing import Callable, Optional
from enum import Enum


class Operation(Enum):
    ADD = "+"
    SUB = "-"
    MUL = "*"
    DIV = "/"
    EQ = '=='


# broken: https://github.com/python/mypy/issues/11098
Expr = int | str | tuple[Operation, 'Expr', 'Expr']


def format_expr(e):
    match e:
        case int(val):
            return str(val)
        case str(name):
            return name
        case (op, a, b):
            return f'({format_expr(a)} {op.value} {format_expr(b)})'


def simplify_expr(e):
    match e:
        case (Operation.ADD, int(a), int(b)):
            return a + b
        case (Operation.SUB, int(a), int(b)):
            return a - b
        case (Operation.MUL, int(a), int(b)):
            return a * b
        case (Operation.DIV, int(a), int(b)):
            assert a % b == 0
            return a // b
        case (Operation.EQ, a, int(val)):
            while True:
                print(format_expr((Operation.EQ, a, val)))
                match a:
                    case (Operation.ADD, e1, int(val1)):
                        a = e1
                        val -= val1
                    case (Operation.ADD, int(val1), e1):
                        a = e1
                        val -= val1
                    case (Operation.SUB, e1, int(val1)):
                        a = e1
                        val += val1
                    case (Operation.SUB, int(val1), e1):
                        a = e1
                        val = val1 - val
                    case (Operation.MUL, e1, int(val1)):
                        a = e1
                        assert val % val1 == 0
                        val //= val1
                    case (Operation.MUL, int(val1), e1):
                        a = e1
                        assert val % val1 == 0
                        val //= val1
                    case (Operation.DIV, e1, int(val1)):
                        a = e1
                        val *= val1
                    case _:
                        break
            return (Operation.EQ, a, val)
        case _:
            return e


@dataclass
class Monkey:
    name: str
    formula: int | str | tuple[Operation, str, str]
    expr: Optional[Expr] = None

    def collect(self, collect_other: Callable[[str], Expr]) -> Expr:
        if self.expr is None:
            expr: Expr
            match self.formula:
                case int(val):
                    expr = val
                case str(name):
                    expr = name
                case (op, a, b):
                    expr = (op, collect_other(a), collect_other(b))
            expr = simplify_expr(expr)
            self.expr = expr
        return self.expr


def parse_monkey(s: str) -> Monkey:
    if m := re.match(r"^([a-z]+): (\d+)$", s):
        name = m.group(1)
        val = int(m.group(2))
        return Monkey(name, val)
    m = re.match(r"^([a-z]+): ([a-z]+) (\+|\-|\*|\/) ([a-z]+)$", s)
    assert m, s
    name = m.group(1)
    op = Operation(m.group(3))
    a, b = m.group(2), m.group(4)
    return Monkey(name, (op, a, b))


def run_monkeys(monkeys: list[Monkey], part_two: bool = False) -> Expr:
    mdict = {m.name: m for m in monkeys}

    if part_two:
        root = mdict['root']
        match root.formula:
            case (_, str(a), str(b)):
                root.formula = Operation.EQ, a, b
            case _:
                assert False
        mdict['humn'].formula = 'humn'

    def collect_other(name: str) -> Expr:
        return mdict[name].collect(collect_other)

    return collect_other('root')


def main():
    lines = sys.stdin.readlines()
    monkeys = [parse_monkey(m.rstrip()) for m in lines]
    e = run_monkeys(monkeys)
    print(format_expr(e))

    monkeys = [parse_monkey(m.rstrip()) for m in lines]
    e2 = run_monkeys(monkeys, part_two=True)
    print(format_expr(e2))


if __name__ == '__main__':
    main()
