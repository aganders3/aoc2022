import pytest


def snafu_to_decimal(snafu: str) -> int:
    res = 0
    for i, d in enumerate(reversed(snafu)):
        if d == "-":
            res -= 5**i
        elif d == "=":
            res -= 2 * 5**i
        elif d.isnumeric():
            res += int(d) * 5**i
        else:
            raise ValueError(f"bad input {d}")
    return res


def decimal_to_snafu(d: int) -> str:
    if d == 0:
        return "0"

    snafu: list[str] = []

    while d > 0:
        q, r = divmod(d, 5)
        if r in (0, 1, 2):
            snafu.append(str(r))
            carry = 0
        elif r == 3:
            snafu.append("=")
            carry = 1
        elif r == 4:
            snafu.append("-")
            carry = 1
        else:
            assert False, "unreachable"
        d = q + carry

    return "".join(reversed(snafu))


EXAMPLE_SNAFU_TO_DECIMAL = (
    ("1=-0-2", 1747),
    ("12111", 906),
    ("2=0=", 198),
    ("21", 11),
    ("2=01", 201),
    ("111", 31),
    ("20012", 1257),
    ("112", 32),
    ("1=-1=", 353),
    ("1-12", 107),
    ("12", 7),
    ("1=", 3),
    ("122", 37),
)


@pytest.fixture(
    params=EXAMPLE_SNAFU_TO_DECIMAL,
    ids=[f"{snafu} <:> {dec}" for snafu, dec in EXAMPLE_SNAFU_TO_DECIMAL],
)
def example_snafu_to_decimal(request) -> tuple[str, int]:
    return request.param


def test_snafu_to_decimal(example_snafu_to_decimal: tuple[str, int]):
    snafu, decimal = example_snafu_to_decimal
    assert snafu_to_decimal(snafu) == decimal


def test_decimal_to_snafu(example_snafu_to_decimal: tuple[str, int]):
    snafu, decimal = example_snafu_to_decimal
    assert decimal_to_snafu(decimal) == snafu


def part_1(fname: str) -> str:
    with open(fname) as f:
        s = sum(snafu_to_decimal(snafu.strip()) for snafu in f)
    return decimal_to_snafu(s)


def test_part_1():
    assert part_1("test.input.txt") == "2=-1=0"


if __name__ == "__main__":
    print("Part 1:", "'" + part_1("input.txt") + "'")
    print("Part 2:", "Mercy!")
