with open(0) as f:
    input = f.read().rstrip("\n")

## -- Part One -- ##

total = 0

for ch in input:
    match ch:
        case "(":
            total += 1
        case ")":
            total -= 1
        case other:
            raise RuntimeError(ch)

print(total)

## -- Part Two -- ##

floor = 0

for pos, ch in enumerate(input, 1):
    match ch:
        case "(":
            floor += 1
        case ")":
            floor -= 1
        case other:
            raise RuntimeError(ch)

    if floor == -1:
        print(pos)
        break
