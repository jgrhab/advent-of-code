input = 36000000

bound = int(input / 10)


# -- Part One -- #

presents = [0] * bound
for elf in range(1, bound):
    for pos in range(elf, len(presents), elf):
        presents[pos] += elf

m = min((ind for ind, pres in enumerate(presents) if pres >= bound))
print(m)


# -- Part Two -- #

# Having `elf` go up to `bound` only is arbitrary.

presents = [0] * (50 * bound)
for elf in range(1, bound):
    for pos in range(elf, 50 * elf + 1, elf):
        presents[pos] += elf * 11

m = min((ind for ind, pres in enumerate(presents) if pres >= input))
print(m)
