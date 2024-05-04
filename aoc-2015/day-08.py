import re

with open(0) as f:
    lines = f.read().splitlines()


## -- Part One -- ##

total = 0

for line in lines:
    matches = re.findall(r"(\\x[0-9a-f]{2}|\\\\|\\\")", line)
    total += sum(map(len, matches)) - len(matches) + 2

print(total)


## -- Part Two -- ##

total = 0

for line in lines:
    total += line.count("\\") + line.count('"') + 2

print(total)
