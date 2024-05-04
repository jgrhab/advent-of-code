with open(0) as f:
    lines = f.read().splitlines()

dimensions = [list(map(int, line.split("x"))) for line in lines]

## -- Part One -- ##

total = 0

for dims in dimensions:
    sides = [dims[0] * dims[1], dims[0] * dims[2], dims[1] * dims[2]]
    total += 2 * sum(sides) + min(sides)

print(total)


## -- Part Two -- ##

total = 0

for dims in dimensions:
    bow_len = dims[0] * dims[1] * dims[2]
    dims.remove(max(dims))
    total += 2 * sum(dims) + bow_len

print(total)
