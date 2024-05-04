with open(0) as f:
    lines = f.read().splitlines()

sues: dict[int, dict[str, int]] = dict()

for ind, line in enumerate(lines):
    sues[ind + 1] = dict()
    split = [s.rstrip(",:") for s in line.split()][2:]
    for k in range(0, 6, 2):
        sues[ind + 1][split[k]] = int(split[k + 1])

mfcsam = {
    "children": 3,
    "cats": 7,
    "samoyeds": 2,
    "pomeranians": 3,
    "akitas": 0,
    "vizslas": 0,
    "goldfish": 5,
    "trees": 3,
    "cars": 2,
    "perfumes": 1,
}


## -- Part One -- ##


def is_match_1(sue: dict[str, int]) -> bool:
    for key, val in sue.items():
        if mfcsam[key] != val:
            return False
    return True


for id in sues:
    if is_match_1(sues[id]):
        print(id)


## -- Part Two -- ##


def is_match_2(sue: dict[str, int]) -> bool:
    for key, val in sue.items():
        if key in ["cats", "trees"]:
            if mfcsam[key] >= val:
                return False
        elif key in ["pomeranians", "goldfish"]:
            if mfcsam[key] <= val:
                return False
        else:
            if mfcsam[key] != val:
                return False

    return True


for id in sues:
    if is_match_2(sues[id]):
        print(id)
