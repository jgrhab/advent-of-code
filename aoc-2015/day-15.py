from dataclasses import dataclass
from math import prod

with open(0) as f:
    lines = f.read().splitlines()


@dataclass
class Ingredient:
    name: str
    properties: tuple[int, int, int, int]
    calories: int

    def __init__(self, s: str):
        sp = s.split()
        self.name = sp[0][:-1]
        self.properties = (
            int(sp[2][:-1]),
            int(sp[4][:-1]),
            int(sp[6][:-1]),
            int(sp[8][:-1]),
        )
        self.calories = int(sp[-1])


ingredients = [Ingredient(line) for line in lines]


def score(coefs: tuple[int, int, int, int]) -> int:
    props: list[list[int]] = [[], [], [], []]
    for ind in range(4):
        props[ind] = [coefs[ind] * p for p in ingredients[ind].properties]

    return prod([max(0, sum(t)) for t in zip(props[0], props[1], props[2], props[3])])


## -- Part One -- ##

coefficients = [
    (x1, x2, x3, 100 - x1 - x2 - x3)
    for x1 in range(101)
    for x2 in range(101 - x1)
    for x3 in range(101 - x1 - x2)
]

max_score = 0

for coefs in coefficients:
    max_score = max(max_score, score(coefs))

print(max_score)


## -- Part Two -- ##


def calories(coefs: tuple[int, int, int, int]) -> int:
    return sum([coefs[ind] * ingredients[ind].calories for ind in range(4)])


new_coefficients = list(filter(lambda c: calories(c) == 500, coefficients))

max_score = 0

for coefs in new_coefficients:
    max_score = max(max_score, score(coefs))

print(max_score)
