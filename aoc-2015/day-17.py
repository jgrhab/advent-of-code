with open(0) as f:
    lines = f.read().splitlines()

containers = [int(line) for line in lines]


def partitions(ls: list[int], target: int) -> list[list[int]]:
    if target < 0:
        return []
    elif target == 0:
        return [[]]

    pts = []
    for ind in range(len(ls)):
        pts_ind = partitions(ls[ind + 1 :], target - ls[ind])
        pts_ind = [[ls[ind]] + c for c in pts_ind]
        pts += pts_ind

    return pts


# -- Part One -- #

pts = partitions(containers, 150)

print(len(pts))


# -- Part Two -- #

min_len = min([len(pt) for pt in pts])
min_pts = [pt for pt in pts if len(pt) == min_len]

print(len(min_pts))
