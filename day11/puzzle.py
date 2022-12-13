from __future__ import annotations
from collections import deque
from functools import reduce
from operator import mul
from typing import Iterable

monkey_string = """\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
"""

class Monkey:
    def __init__(
        self,
        items: deque[int],
        op: Callable[[int], int],
        test: int,
        true_target: int,
        false_target: int,
    ):
        self.items = items
        self.inspect = op
        self.inspected = 0
        self.test = test
        self.true_target = true_target
        self.false_target = false_target

    @staticmethod
    def parse(s: str) -> Monkey:
        """This parser is probably a crime"""
        for line in s.splitlines():
            match line.strip().split(": "):
                case ["Starting items", l]:
                    items = deque(map(int, l.split(", ")))
                case ["Operation", o]:
                    # haha!
                    exec(f"def op(old): return {o.split(' = ')[1]}", globals())
                case ["Test", t]:
                    test = int(t.split(" by ")[1])
                case ["If true", t]:
                    true_target = int(t.split(" monkey ")[1])
                case ["If false", t]:
                    false_target = int(t.split(" monkey ")[1])
        return Monkey(items, op, test, true_target, false_target)

    def take_turn(self, worry_reduction=None) -> Iterable[tuple[int, int]]:
        while self.items:
            item = self.items.popleft()
            item = self.inspect(item)
            self.inspected += 1
            if worry_reduction is None:
                # for part 1
                item = item // 3
            else:
                item = item % worry_reduction
            target = self.false_target if item % self.test else self.true_target
            yield (item, target)


def part_1(input_file: str, *, debug=False) -> int:
    with open(input_file) as f:
        monkeys = [
            Monkey.parse(s)
            for s in f.read().split("\n\n")
        ]

    for _ in range(20):
        for m in monkeys:
            for item, target in m.take_turn():
                monkeys[target].items.append(item)

    if debug:
        for i, m in enumerate(monkeys):
            print(f"Monkey {i}: {', '.join(map(str, m.items))}")
            print(f"\tinspected items {m.inspected} times")

    monkeys.sort(key=lambda m: m.inspected, reverse=True)
    return monkeys[0].inspected * monkeys[1].inspected


def part_2(input_file: str, *, debug=False) -> int:
    with open(input_file) as f:
        monkeys = [
            Monkey.parse(s)
            for s in f.read().split("\n\n")
        ]

    # take the product of all the monkey tests
    worry_reduction = reduce(mul, (m.test for m in monkeys), 1)

    for _ in range(10_000):
        for m in monkeys:
            for item, target in m.take_turn(worry_reduction):
                monkeys[target].items.append(item)

    if debug:
        for i, m in enumerate(monkeys):
            print(f"Monkey {i}: {', '.join(map(str, m.items))}")
            print(f"\tinspected items {m.inspected} times")

    monkeys.sort(key=lambda m: m.inspected, reverse=True)
    return monkeys[0].inspected * monkeys[1].inspected


def test_part_1():
    assert part_1("test.input.txt") == 10605

def test_part_2():
    assert part_2("test.input.txt") == 2713310158


if __name__ == "__main__":
    print("Part 1:", part_1("input.txt"))
    print("Part 2:", part_2("input.txt"))
