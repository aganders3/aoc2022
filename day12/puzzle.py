import textwrap
from collections import deque
from itertools import product
from math import inf
from typing import Iterable, NamedTuple

class Node(NamedTuple):
    row: int
    col: int
    height: str


def neighbors(i: int, j: int, m: int, n: int) -> Iterable[tuple[int, int]]:
    yield from (
        (i + di, j + dj)
        for di, dj in ((0, 1), (1, 0), (0, -1), (-1, 0))
        if i + di in range(m) and j + dj in range(n)
    )

    
class Grid:
    def __init__(self, s: str):
        s = s.splitlines()
        rows = len(s)
        cols = len(s[0])
        self.adjacency_list: dict[Node, list[Node]] = {}

        self.nodes: dict[tuple[int, int], Node] = {}
        for i, j in product(range(rows), range(cols)):
            start = s[i][j] == "S"
            end = s[i][j] == "E"
            height = "a" if start else "z" if end else s[i][j]
            n = Node(i, j, height)
            if start:
                self.start = n
            if end:
                self.end = n
            self.nodes[(i, j)] = n

        for node in self.nodes.values():
            self.adjacency_list.setdefault(node, []).extend(
                self.nodes[(i, j)]
                for i, j in neighbors(node.row, node.col, rows, cols)
                if ord(self.nodes[(i, j)].height) <= ord(node.height) + 1
            )

    def neighbors(self, n: Node) -> Iterable[Node]:
        yield from self.adjacency_list[n]

    def shortest_path(self, start: Node | None = None) -> tuple[list[Node], int]:
        """return the shortest path from start to end + number of steps on path"""
        if start is None:
            start = self.start
        visited = set()
        bfs_q = deque([[start]])

        while bfs_q:
            path = bfs_q.popleft()
            node = path[-1]
            if node is self.end:
                break

            if node not in visited:
                for n in self.adjacency_list[node]:
                    p = list(path)
                    p.append(n)
                    bfs_q.append(p)
                visited.add(node)
        else:
            return None, inf

        return path, len(path) - 1


def test_part_1():
    ex = textwrap.dedent("""\
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    """)

    g = Grid(ex)
    _, steps = g.shortest_path()
    
    assert steps == 31

def test_part_2():
    ex = textwrap.dedent("""\
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    """)

    g = Grid(ex)
    min_steps = inf
    for node in g.nodes.values():
        if node.height == "a":
            _, steps = g.shortest_path(node)
            min_steps = min(steps, min_steps) 
    assert min_steps == 29


if __name__ == "__main__":
    with open("input.txt") as f:
        s = f.read()
    g = Grid(s)
    _, steps = g.shortest_path()
    print("Part 1:", steps)

    min_steps = inf
    for node in g.nodes.values():
        if node.height == "a":
            _, steps = g.shortest_path(node)
            min_steps = min(steps, min_steps) 
    print("Part 2:", min_steps)
