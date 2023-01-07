import copy
import math
from collections import deque
from dataclasses import dataclass
from typing import Iterable


@dataclass
class Positions:
    height: int
    width: int
    start: tuple[int, int]
    end: list[tuple[int, int]]
    elf: tuple[int, int]
    positions: dict[tuple[int, int], list[str]]

    def all_blizzards(self) -> list[dict[tuple[int, int], list[str]]]:
        # print_debug(self)
        blizzards = []
        for _ in range(math.lcm(self.width, self.height)):
            blizzards.append(self.positions)
            self.step_blizzards()
        # print_debug(self)
        return blizzards

    def step_blizzards(self):
        next_positions = {}
        for loc, blizzards in self.positions.items():
            r, c = loc
            for b in blizzards:
                if b == "<":
                    next_positions.setdefault((r, (c - 1) % self.width), []).append("<")
                if b == "^":
                    next_positions.setdefault(((r - 1) % self.height, c), []).append("^")
                if b == ">":
                    next_positions.setdefault((r, (c + 1) % self.width), []).append(">")
                if b == "v":
                    next_positions.setdefault(((r + 1) % self.height, c), []).append("v")
        self.positions = next_positions

    def elf_moves(self) -> Iterable[tuple[int, int]]:
        """All possible moves for the elf, including waiting."""
        r, c = self.elf
        for dr, dc in ((0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)):
            next_r, next_c = r + dr, c + dc

            # stop if we can go into the end
            if (next_r, next_c) == self.end[-1]:
                yield (next_r, next_c)
                break

            # it's okay to wait at the start
            if (r, c) == self.start and (next_r, next_c) == self.start:
                yield (next_r, next_c)

            # but otherwise never return to the start
            if (r, c) != self.start and (next_r, next_c) == self.start:
                continue

            # keep it in bounds
            if next_r not in range(self.height) or next_c not in range(self.width):
                continue

            # don't walk into a blizzard
            if not self.positions.get((r + dr, c + dc)):
                yield (next_r, next_c)


def parse(fname: str) -> Positions:
    with open(fname) as f:
        data = [line.strip() for line in f.readlines()]
    height, width = len(data) - 2, len(data[0]) - 2
    positions = {
        (row, col): [char]
        for row, line in enumerate(data, start=-1)
        for col, char in enumerate(line, start=-1)
        if char not in ("#", ".")
    }
    start = (-1, data[0].index(".") - 1)
    end = (height, data[-1].index(".") - 1)

    return Positions(height, width, start, [end], start, positions)


def print_debug(positions: Positions):
    for r in range(-1, positions.height + 1):
        print(f"{r:3}: ", end="")
        for c in range(-1, positions.width + 1):
            x = positions.positions.get((r, c), ["."])
            if (r, c) == positions.elf:
                print("E", end="")
            elif (r, c) not in (positions.start, positions.end) and (
                r in (-1, positions.height) or c in (-1, positions.width)
            ):
                print("#", end="")
            elif len(x) == 1:
                print(x[0], end="")
            else:
                print(len(x), end="")
        print()


def part_1(initial_positions: Positions) -> int:
    blizzards = initial_positions.all_blizzards()

    to_explore = deque([(
        0, 
        Positions(
            initial_positions.height,
            initial_positions.width,
            initial_positions.start,
            initial_positions.end,
            initial_positions.elf,
            blizzards[0],
        ),
    )])
    seen = set()

    best: list[int] = []
    while to_explore:
        steps, pos = to_explore.popleft()

        h = (steps % len(blizzards), pos.elf,)
        if h in seen:
            continue
        else:
            seen.add(h)

        if pos.elf == pos.end[-1]:
            best.append(steps - sum(best))
            pos.start = pos.end.pop()
            to_explore.clear()
            seen.clear()

        if not pos.end:
            break

        n = (steps + 1) % len(blizzards)
        pos.positions = blizzards[n]
        for e in pos.elf_moves():
            to_explore.append((
                steps + 1, 
                Positions(
                    pos.height,
                    pos.width,
                    pos.start,
                    pos.end,
                    e,
                    pos.positions,
                ),
            ))

    return sum(best)


def part_2(initial_positions: Positions) -> int:
    initial_positions.end += [initial_positions.start, initial_positions.end[0]]
    return part_1(initial_positions)


def test_part_1():
    assert part_1(parse("test.input.txt")) == 18


def test_part_2():
    assert part_2(parse("test.input.txt")) == 54


if __name__ == "__main__":
    print("Part 1:", part_1(parse("input.txt")))
    print("Part 2:", part_2(parse("input.txt")))
