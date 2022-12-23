import copy
import itertools
from collections import defaultdict, deque
from enum import auto, IntEnum
from dataclasses import dataclass, field

import pytest


class Resource(IntEnum):
    GEODE = auto()
    OBSIDIAN = auto()
    CLAY = auto()
    ORE = auto()

    @classmethod
    def all(cls):
        return cls.__members__.values()


@dataclass
class RobotBlueprint:
    ore_robot_cost: int
    clay_robot_cost: int
    obsidian_robot_cost: tuple[int, int]
    geode_robot_cost: tuple[int, int]

    def cost(self, resource: Resource) -> dict[Resource, int]:
        match resource:
            case Resource.ORE:
                return {Resource.ORE: self.ore_robot_cost}
            case Resource.CLAY:
                return {Resource.ORE: self.clay_robot_cost}
            case Resource.OBSIDIAN:
                return dict(
                    zip((Resource.ORE, Resource.CLAY), self.obsidian_robot_cost)
                )
            case Resource.GEODE:
                return dict(
                    zip((Resource.ORE, Resource.OBSIDIAN), self.geode_robot_cost)
                )


@dataclass
class State:
    blueprint: RobotBlueprint
    time_remaining: int = 24
    resources: defaultdict[Resource, int] = field(
        init=False, default_factory=lambda: defaultdict(int)
    )
    robots: defaultdict[Resource, int] = field(
        init=False, default_factory=lambda: defaultdict(int)
    )
    pending_robots: defaultdict[Resource, int] = field(
        init=False, default_factory=lambda: defaultdict(int)
    )

    @property
    def hashable(self):
        return (
            self.time_remaining,
            tuple(self.resources.values()), 
            tuple(self.robots.values()), 
            tuple(self.pending_robots.values()), 
        )

    def __post_init__(self):
        self.robots[Resource.ORE] = 1

    def tic(self):
        for resource in Resource.all():
            self.resources[resource] += self.robots[resource]
            self.robots[resource] += self.pending_robots[resource]
            self.pending_robots[resource] = 0

        self.time_remaining -= 1

        return self

    def build_robot(self, resource: Resource):
        cost = self.blueprint.cost(resource)

        for r, amount in cost.items():
            if self.resources[r] < amount:
                return None

        for r, amount in cost.items():
            self.resources[r] -= amount

        self.pending_robots[resource] += 1
        return self


def get_max(state: State):
    best = -1
    seen: set[tuple[int, tuple[int, int, int], tuple[int, int, int], tuple[int, int, int]]] = set()
    todo = deque([state])
    while todo:
        cur = todo.popleft()
        if cur.hashable in seen:
            continue
        seen.add(cur.hashable)
        best = max(cur.resources[Resource.GEODE], best)

        if cur.time_remaining > 0:
            for resource in Resource.all():
                if new_state := copy.deepcopy(cur).build_robot(resource):
                    todo.append(new_state.tic())
                # bought a geode robot? don't explore other paths
                if new_state and resource == Resource.GEODE:
                    break
            else:
                todo.append(cur.tic())

    return best


def test_part_1():
    b = RobotBlueprint(4, 2, (3, 14), (2, 7))
    s = State(b, time_remaining=24)
    assert get_max(s) == 9

    b = RobotBlueprint(2, 3, (3, 8), (3, 12))
    s = State(b, time_remaining=24)
    assert get_max(s) == 12
