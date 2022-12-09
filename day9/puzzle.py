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

class LongRope:
    def __init__(self, length: int = 10):
        self.knots: list[tuple[int, int]] = [(0, 0)] * length
        self.visited_by_tail: set[tuple[int, int]] = { self.tail }

    def move_head(self, direction: str, n: int) -> None:
        for _ in range(n):
            self._move_head(direction)
            self._move_tail()

    @property
    def head(self):
        return self.knots[0]

    @head.setter
    def head(self, val):
        self.knots[0] = val

    @property
    def tail(self):
        return self.knots[-1]

    def _move_head(self, direction: str) -> None:
        dx, dy = DIRECTION[direction]
        self.head = (self.head[0] + dx, self.head[1] + dy)

    def _move_tail(self) -> None:
        for i, knot in enumerate(self.knots[1:], start=1):
            dx = self.knots[i - 1][0] - self.knots[i][0]
            dy = self.knots[i - 1][1] - self.knots[i][1]

            if 0 in (dx, dy):
                self.knots[i] = (
                    self.knots[i][0] + int(copysign(abs(dx) // 2, dx)),
                    self.knots[i][1] + int(copysign(abs(dy) // 2, dy)),
                )
            elif 2 in (abs(dx), abs(dy)):
                self.knots[i] = (
                    self.knots[i][0] + int(copysign(max(1, abs(dx) // 2), dx)),
                    self.knots[i][1] + int(copysign(max(1, abs(dy) // 2), dy)),
                )

        self.visited_by_tail.add(self.tail)


def part_1(input_filename: str) -> int:
    rope = Rope()
    for move in open(input_filename):
        direction, n = move.strip().split(" ")
        rope.move_head(direction, int(n))
    return len(rope.visited_by_tail)

def part_2(input_filename: str) -> int:
    rope = LongRope()
    for move in open(input_filename):
        direction, n = move.strip().split(" ")
        rope.move_head(direction, int(n))
    return len(rope.visited_by_tail)


def test_part_1():
    assert part_1("test.input.txt") == 13

def test_part_2():
    assert part_2("test.input.2.txt") == 36


if __name__ == "__main__":
    print("Part 1:", part_1("input.txt"))
    print("Part 2:", part_2("input.txt"))
