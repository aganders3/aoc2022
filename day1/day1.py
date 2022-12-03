# part 1
max_index = max_cals = 0
elves = [0]
for line in open("input.txt"):
    l = line.strip()
    if l.isnumeric():
        elves[-1] += int(l)
    else:
        elves.append(0)

    if elves[-1] > max_cals:
        max_cals = elves[-1]
        max_index = len(elves) - 1
print("Part 1", max_cals)

# part 2
elves = [0]
for line in open("input.txt"):
    l = line.strip()
    if l.isnumeric():
        elves[-1] += int(l)
    else:
        elves.append(0)
elves = sorted(elves)
print("Part 2", sum(elves[-3:]))
