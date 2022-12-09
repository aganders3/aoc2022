from math import copysign


DIRECTION = {
    "U": (0, 1),
    "D": (0, -1),
    "L": (-1, 0),
    "R": (1, 0),
}

class Rope:
    def __init__(self):
        self.head: tuple[int, int] = (0, 0)
        self.tail: tuple[int, int] = (0, 0)
        self.visited_by_tail: set[tuple[int, int]] = { self.tail }

    def move_head(self, direction: str, n: int) -> None:
        for _ in range(n):
            self._move_head(direction)
            self._move_tail()

    def _move_head(self, direction: str) -> None:
        dx, dy = DIRECTION[direction]
        self.head = (self.head[0] + dx, self.head[1] + dy)

    def _move_tail(self) -> None:
        # head and tail are already touching, do nothing
        # if (
        #     abs(self.head[0] - self.tail[0]) < 2
        #     and abs(self.head[1] - self.tail[1]) < 2
        # ):
        #     pass

        dx = self.head[0] - self.tail[0]
        dy = self.head[1] - self.tail[1]

        if 0 in (dx, dy):
            self.tail = (
                self.tail[0] + int(copysign(abs(dx) // 2, dx)),
                self.tail[1] + int(copysign(abs(dy) // 2, dy)),
            )
        elif 2 in (abs(dx), abs(dy)):
            self.tail = (
                self.tail[0] + int(copysign(max(1, abs(dx) // 2), dx)),
                self.tail[1] + int(copysign(max(1, abs(dy) // 2), dy)),
            )

        self.visited_by_tail.add(self.tail)


def part_1(input_filename: str) -> int:
    rope = Rope()
    for move in open(input_filename):
        direction, n = move.strip().split(" ")
        rope.move_head(direction, int(n))
    return len(rope.visited_by_tail)


def test_part_1():
    assert part_1("test.input.txt") == 13


if __name__ == "__main__":
    print("Part 1:", part_1("input.txt"))
    # print("Part 1:", part_1("input.txt"))
