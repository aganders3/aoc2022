import re
from typing import Iterable, Tuple

import pytest


def parse_moves(s: str) -> Iterable[Tuple[int, int, int]]:
    p = re.compile(r"^move ([1-9][0-9]*) from ([1-9][0-9]*) to ([1-9][0-9]*)$")
    for move in s.splitlines():
        if m := p.match(move):
            yield int(m[1]), int(m[2]), int(m[3])

def parse_stacks(s: str) -> Tuple[list[str], ...]:
    stack_data = s.splitlines()
    stacks: list[Tuple[int, list]] = [(i, []) for i, n in enumerate(stack_data[-1]) if n.isnumeric()]
    for row in stack_data[-1::-1]:
        for stack in stacks:
            if row[stack[0]].isalpha():
                stack[1].append(row[stack[0]])
    return tuple(s[1] for s in stacks)

def apply_move_9000(stacks: Tuple[list[str], ...], move: Tuple[int, int, int]):
    n, o, d = move
    stacks[d - 1].extend(stacks[o - 1].pop() for _ in range(n))

def apply_move_9001(stacks: Tuple[list[str], ...], move: Tuple[int, int, int]):
    n, o, d = move
    stacks[d - 1].extend(reversed([stacks[o - 1].pop() for _ in range(n)]))

def part_1(s: str):
    stack_data, move_data = s.split("\n\n")
    stacks = parse_stacks(stack_data)
    moves = parse_moves(move_data)
    for move in moves:
        apply_move_9000(stacks, move)
    return "".join(s[-1] for s in stacks)

def part_2(s: str):
    stack_data, move_data = s.split("\n\n")
    stacks = parse_stacks(stack_data)
    moves = parse_moves(move_data)
    for move in moves:
        apply_move_9001(stacks, move)
    return "".join(s[-1] for s in stacks)


@pytest.fixture
def sample_input():
    import textwrap
    return textwrap.dedent(
        """\
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 
        
        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
        """
    )

@pytest.fixture
def sample_input_stacks(sample_input):
    return sample_input.split("\n\n")[0]

@pytest.fixture
def sample_input_moves(sample_input):
    return sample_input.split("\n\n")[1]

def test_parse_stacks(sample_input_stacks):
    stacks = parse_stacks(sample_input_stacks)
    assert stacks[0] == ["Z", "N"]
    assert stacks[1] == ["M", "C", "D"]
    assert stacks[2] == ["P"]

def test_parse_moves(sample_input_moves):
    moves = list(parse_moves(sample_input_moves))
    assert moves[0] == (1, 2, 1)
    assert moves[1] == (3, 1, 3)
    assert moves[2] == (2, 2, 1)
    assert moves[3] == (1, 1, 2)

def test_apply_move(sample_input_stacks):
    stacks = parse_stacks(sample_input_stacks)
    move = (1, 2, 1)
    apply_move_9000(stacks, move)
    assert stacks[0] == ["Z", "N", "D"]
    assert stacks[1] == ["M", "C"]
    assert stacks[2] == ["P"]

def test_apply_move_9001(sample_input_stacks):
    stacks = parse_stacks(sample_input_stacks)
    move = (1, 2, 1)
    apply_move_9000(stacks, move)
    move = (3, 1, 3)
    apply_move_9001(stacks, move)
    assert stacks[0] == []
    assert stacks[1] == ["M", "C"]
    assert stacks[2] == ["P", "Z", "N", "D"]

def test_part_1(sample_input):
    assert part_1(sample_input) == "CMZ"

def test_part_2(sample_input):
    assert part_2(sample_input) == "MCD"
    
if __name__ == "__main__":
    from pathlib import Path
    input_file = Path("./input.txt")
    print("Part 1:", part_1(input_file.read_text()))
    print("Part 2:", part_2(input_file.read_text()))
