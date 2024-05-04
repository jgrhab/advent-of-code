import json

with open(0) as f:
    input = json.load(f)


def count(thing: int | str | list | dict, ignored_val: str) -> int:
    if isinstance(thing, (int,)):
        return thing

    elif isinstance(thing, (list,)):
        return sum([count(x, ignored_val) for x in thing])

    elif isinstance(thing, (dict,)):
        if ignored_val in thing.values():
            return 0
        else:
            return sum([count(val, ignored_val) for val in thing.values()])

    return 0


## -- Part One -- ##

total = 0

for thing in input:
    total += count(thing, "")

print(total)


## -- Part Two -- ##

total = 0

for thing in input:
    total += count(thing, "red")

print(total)
