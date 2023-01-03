from collections import deque
from functools import partial
from itertools import count, product


def parse(fname: str) -> set[tuple[int, int]]:
    return {
        (row, col)
        for row, line in enumerate(open(fname))
        for col, char in enumerate(line)
        if char == "#"
    }


def debug_print(positions: set[tuple[int, int]]):
    for i in range(
        min(row for row, col in positions), max(row for row, col in positions) + 1
    ):
        for j in range(
            min(col for row, col in positions), max(col for row, col in positions) + 1
        ):
            if (i, j) in positions:
                print("#", end="")
            else:
                print(".", end="")
        print("")


def bounds(positions: set[tuple[int, int]]) -> tuple[int, int]:
    width = max(col for row, col in positions) - min(col for row, col in positions)
    height = max(row for row, col in positions) - min(row for row, col in positions)
    return (width, height)


def get_proposals(
    positions: set[tuple[int, int]], i: int
) -> dict[tuple[int, int], list[tuple[int, int]]]:
    # defined here to capture `positions`
    def _proposal(d: str, p: tuple[int, int]) -> tuple[int, int] | None:
        r, c = p
        if d == "N" and not any((r - 1, c + dc) in positions for dc in (-1, 0, 1)):
            return (r - 1, c)
        elif d == "S" and not any((r + 1, c + dc) in positions for dc in (-1, 0, 1)):
            return (r + 1, c)
        elif d == "W" and not any((r + dr, c - 1) in positions for dr in (-1, 0, 1)):
            return (r, c - 1)
        elif d == "E" and not any((r + dr, c + 1) in positions for dr in (-1, 0, 1)):
            return (r, c + 1)
        return None

    directions = deque(("N", "S", "W", "E"))
    directions.rotate(-i % 4)
    checks = [partial(_proposal, d) for d in directions]
    proposals: dict[tuple[int, int], list[tuple[int, int]]] = {}
    for p in positions:
        r, c = p
        if not any(
            (r + dr, c + dc) in positions
            for dr, dc in product((-1, 0, 1), (-1, 0, 1))
            if (dr, dc) != (0, 0)
        ):
            continue
        for check in checks:
            if proposed := check(p):
                proposals.setdefault(proposed, []).append(p)
                break
    return proposals


def part_1(initial_positions: set[tuple[int, int]]) -> int:
    positions = initial_positions

    for i in range(10):
        proposals = get_proposals(positions, i)
        for nxt, cur in proposals.items():
            if len(cur) == 1:
                positions.remove(cur[0])
                positions.add(nxt)

    w, h = bounds(positions)
    # debug_print(positions)
    return (w + 1) * (h + 1) - len(positions)


def part_2(initial_positions: set[tuple[int, int]]) -> int:
    positions = initial_positions

    for i in count():
        moved = False
        proposals = get_proposals(positions, i)
        for nxt, cur in proposals.items():
            if len(cur) == 1:
                moved = True
                positions.remove(cur[0])
                positions.add(nxt)
        if not moved:
            break

    return i + 1


def test_part_1():
    assert part_1(parse("test.input.txt")) == 110


def test_part_2():
    assert part_2(parse("test.input.txt")) == 20


if __name__ == "__main__":
    print("Part 1:", part_1(parse("input.txt")))
    print("Part 2:", part_2(parse("input.txt")))
