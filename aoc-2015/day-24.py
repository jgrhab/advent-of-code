from itertools import combinations
from math import prod

with open(0) as f:
    packages = set([int(x) for x in f.read().splitlines()])


def min_len_gp(st: set[int], num_gp: int) -> int:
    """Computes the minimal length that a group must have to be part of an admissible
    partition of the set `st` into `num_gp` subsets.
    This gives a lower bound for the size of the first group of admissible paritions."""

    gp_weight = int(sum(st) / num_gp)

    for ln in range(1, len(st) - num_gp + 1):
        for gp in combinations(st, ln):
            if sum(gp) == gp_weight:
                return ln

    return 0


def is_groupable(st: set[int], num_gp: int) -> bool:
    """Checks whether the set `st` can be partitioned into `num_gp` subsets
    whose sum (i.e. the sum of their elements) are all equal."""

    if num_gp == 1:
        return True

    gp_weight = int(sum(st) / num_gp)
    min_len = min_len_gp(st, num_gp)

    for ln in range(min_len, len(st) - num_gp + 1):
        for gp in (gp for gp in combinations(st, ln) if sum(gp) == gp_weight):
            if is_groupable(st - set(gp), num_gp - 1):
                return True

    return False


def admissible_groups(st: set[int], num_gp: int) -> set[tuple[int, ...]]:
    """Returns a set containing all possible first groups which are part of an
    admissible partition of the set `st` into `num_gp` subsets.
    A partition is admissible if there exist gp_1, ..., gp_k (k = `num_gp`) such that
    sum(gp_i) = sum(st) / num_gp.
    The first group of an admissible partition is the one with the least number
    of elements."""

    admissible = set()

    gp_weight = int(sum(st) / num_gp)
    min_len = min_len_gp(st, num_gp)

    for gp in (gp for gp in combinations(st, min_len) if sum(gp) == gp_weight):
        if is_groupable(st - set(gp), num_gp - 1):
            admissible.add(gp)

    return admissible


# -- Part One -- #

ad_gps = admissible_groups(packages, 3)
qes = {prod(gp) for gp in ad_gps}
print(min(qes))


# -- Part Two -- #

ad_gps = admissible_groups(packages, 4)
qes = {prod(gp) for gp in ad_gps}
print(min(qes))
