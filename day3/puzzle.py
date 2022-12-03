import string
from dataclasses import dataclass

import pytest


priority = {
    letter: i
    for i, letter in enumerate(string.ascii_letters, start=1)
}


class Rucksack:
    def __init__(self, contents: str):
        n = len(contents)
        assert n % 2 == 0, f"uneven number of items in '{contents}': {len(contents)}"
        self.first_compartment = contents[:n//2]
        self.second_compartment = contents[n//2:]

    @property
    def common_item(self) -> str:
        common = set(self.first_compartment) & set(self.second_compartment)
        n = len(common)
        assert n == 1, f"invalid rucksack, expected 1 common item, found {n}"
        return common.pop()

    @property
    def unique_contents(self) -> set[str]:
        return set(self.first_compartment + self.second_compartment)


@pytest.mark.parametrize(
    "first,second",
    (
        ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
        ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
    ),
)
def test_create_rucksack(first, second):
    r = Rucksack(first + second)
    assert r.first_compartment == first
    assert r.second_compartment == second

@pytest.fixture
def rucksacks():
    contents = """\
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
        """
    return [
        Rucksack(c.strip())
        for c in contents.splitlines()
        if c.strip()
    ]


def test_part_1(rucksacks):
    assert part_1(rucksacks) == 157
    
def test_part_2(rucksacks):
    group_1 = rucksacks[:3]
    group_2 = rucksacks[3:]
    assert badge(group_1) == "r"
    assert badge(group_2) == "Z"

    assert part_2(rucksacks) == 70


def part_1(rucksacks: list[Rucksack]) -> int:
    return sum(
        priority[r.common_item]
        for r in rucksacks
    )


def badge(group: list[Rucksack]) -> str:
    e1, e2, e3 = group
    return (
        e1.unique_contents
        & e2.unique_contents
        & e3.unique_contents
    ).pop()


def part_2(rucksacks: list[Rucksack]) -> int:
    result = 0
    rucksacks = rucksacks.copy()
    while rucksacks:
        result += priority[badge(rucksacks.pop() for _ in range(3))]
    return result


if __name__ == "__main__":
    with open("./input.txt") as f:
        rucksacks = [
            Rucksack(c.strip())
            for c in f.readlines()
            if c.strip()
        ]

    print("Part 1:", part_1(rucksacks))
    print("Part 2:", part_2(rucksacks))
