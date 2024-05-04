import itertools
from typing import Dict, Tuple

with open(0) as f:
    lines = f.read().splitlines()

nodes = set()
dist: Dict[Tuple[str, str], int] = {}

for line in lines:
    split = line.split()

    dist[(split[0], split[2])] = int(split[-1])
    dist[(split[2], split[0])] = int(split[-1])

    nodes.add(split[0])
    nodes.add(split[2])

min_len = max(dist.values()) * len(nodes)
max_len = 0

for path in itertools.permutations(nodes):
    path_len = 0
    for ind in range(len(path) - 1):
        path_len += dist[(path[ind], path[ind + 1])]

    min_len = min(min_len, path_len)
    max_len = max(max_len, path_len)


## -- Part One -- ##

print(min_len)

## -- Part Two -- ##

print(max_len)
