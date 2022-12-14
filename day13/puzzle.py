import sys
from collections import deque
from functools import cmp_to_key

import pytest

class CompareResult:
    GOOD = -1
    BAD = 1
    CONTINUE = 0

def parse_pairs(fname: str):
    with open(fname) as f:
        packets = f.read().split("\n\n")
    for p in packets:
        left, right = p.splitlines()
        yield eval(left), eval(right)

def parse(fname: str):
    with open(fname) as f:
        packets = [
            eval(line.strip())
            for line in f.readlines()
            if line.strip()
        ]
    yield from packets

def compare_ints(left, right):
    if left < right:
        return CompareResult.GOOD
    elif left > right:
        return CompareResult.BAD
    else:
        return CompareResult.CONTINUE

def compare_lists(left, right):
    match (left, right):
        case [int(l), int(r)]:
            return compare_ints(left, right)
        case [int(l), list(r)]:
            left = [l]
        case [list(l), int(r)]:
           right = [r]
    
    left, right = deque(left), deque(right)
    while left and right:
        l, r = left.popleft(), right.popleft()
        if (result := compare_lists(l, r)) == CompareResult.CONTINUE:
            continue
        else:
            return result

    # matched values lists exhausted, check remaining
    if left:
        # right exhausted before left
        return CompareResult.BAD
    elif right:
        # left exhausted before right
        return CompareResult.GOOD
    else:
        return CompareResult.CONTINUE

def part_1(fname):
    return sum(
        i
        for i, p in enumerate(parse_pairs(fname), start=1)
        if compare_lists(*p) == CompareResult.GOOD
    ) 

def part_2(fname):
    dividers = [[[2]], [[6]]]
    # print(list(parse(fname)))
    sorted_packets = sorted(
        list(parse(fname)) + dividers,
        key=cmp_to_key(compare_lists),
    )
    return (sorted_packets.index(dividers[0]) + 1) * (sorted_packets.index(dividers[1]) + 1)
    
@pytest.mark.parametrize(
    "left,right,expected",
    (
        ([1,1,3,1,1], [1,1,5,1,1], CompareResult.GOOD),
        ([[1],[2,3,4]], [[1],4], CompareResult.GOOD),
        ([9], [[8,7,6]], CompareResult.BAD),
        ([[4,4],4,4], [[4,4],4,4,4], CompareResult.GOOD),
        ([7,7,7,7], [7,7,7], CompareResult.BAD),
        ([], [3], CompareResult.GOOD),
        ([[[]]], [[]], CompareResult.BAD),
        ([1,[2,[3,[4,[5,6,7]]]],8,9], [1,[2,[3,[4,[5,6,0]]]],8,9], CompareResult.BAD),
    ),
)
def test_compare_lists(left, right, expected):
    assert compare_lists(left, right) == expected

def test_part_1():
    assert part_1("test.input.txt") == 13

def test_part_2():
    assert part_2("test.input.txt") == 140

if __name__ == "__main__":
    print("Part 1:", part_1("input.txt"))
    print("Part 2:", part_2("input.txt"))
