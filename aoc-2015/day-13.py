import itertools


with open(0) as f:
    lines = f.read().splitlines()


people: set[str] = set()
happiness_var: dict[tuple[str, str], int] = dict()

for line in lines:
    split = line.split()

    match split[2]:
        case "gain":
            var = int(split[3])
        case "lose":
            var = -int(split[3])
        case _:
            raise RuntimeError()

    happiness_var[(split[0], split[-1][:-1])] = var
    people.add(split[0])


def perm_happiness(perm: tuple[str, ...], start: str) -> int:
    p = [start] + list(perm) + [start]
    happiness = 0

    for ind in range(len(p) - 1):
        happiness += happiness_var[(p[ind], p[ind + 1])]
        happiness += happiness_var[(p[ind + 1], p[ind])]

    return happiness


## -- Part One -- ##

happiness_max = 0
start = people.pop()

for perm in itertools.permutations(people):
    happiness = perm_happiness(perm, start)
    happiness_max = max(happiness_max, happiness)

print(happiness_max)


## -- Part Two -- ##

happiness_max = 0

people.add(start)
start = "self"
for person in people:
    happiness_var[(start, person)] = 0
    happiness_var[(person, start)] = 0

for perm in itertools.permutations(people):
    happiness = perm_happiness(perm, start)
    happiness_max = max(happiness_max, happiness)

print(happiness_max)
