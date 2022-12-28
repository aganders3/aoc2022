from z3 import Int, Optimize, sat

def parse(fname: str) -> dict[str, int | tuple[str, str, str]]:
    with open(fname) as f:
        monkeys = f.readlines()

    constraints: dict[str, int | tuple[str, str, str]] = {}
    for monkey in monkeys:
        name, rule = monkey.strip().split(": ")
        try:
            constraints[name] = int(rule)
        except ValueError:
            m1, op, m2 = rule.split(" ")
            constraints[name] = (m1, op, m2)

    return constraints

def part_2(constraints):
    o = Optimize()
    z3_vals = {name: Int(name) for name in constraints}
    m1, _, m2 = constraints["root"]

    o.add(z3_vals[m1] == z3_vals[m2])

    for k, v in constraints.items():
        if k in ("humn", "root"):
            continue
        if isinstance(v, int):
            o.add(z3_vals[k] == v)
        else:
            m1, op, m2 = v
            if op == "+":
                o.add(z3_vals[k] == z3_vals[m1] + z3_vals[m2])
            elif op == "-":
                o.add(z3_vals[k] == z3_vals[m1] - z3_vals[m2])
            elif op == "/":
                o.add(z3_vals[k] == z3_vals[m1] / z3_vals[m2])
            elif op == "*":
                o.add(z3_vals[k] == z3_vals[m1] * z3_vals[m2])

    o.minimize(z3_vals["humn"])
    assert o.check() == sat, "not solved!"
    # print(o.model())
    return o.model()[z3_vals["humn"]]

def test_part_2():
    assert part_2(parse("test.input.txt")) == 301

if __name__ == "__main__":
    constraints = parse("input.txt")
    print("Part 2:", part_2(constraints))
