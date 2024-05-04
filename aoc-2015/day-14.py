from dataclasses import dataclass

with open(0) as f:
    lines = f.read().splitlines()


@dataclass
class Reindeer:
    name: str
    speed: int
    t_fly: int
    t_rest: int

    def __init__(self, s: str):
        split = s.split()
        self.name = split[0]
        self.speed = int(split[3])
        self.t_fly = int(split[6])
        self.t_rest = int(split[-2])


def distance(reindeer: Reindeer, time: int) -> int:
    t_cycle = reindeer.t_fly + reindeer.t_rest
    cycles, remainder = int(time / t_cycle), time % t_cycle
    remainder_fly = min(remainder, reindeer.t_fly)

    return cycles * (reindeer.speed * reindeer.t_fly) + remainder_fly * reindeer.speed


reindeers = [Reindeer(line) for line in lines]
duration = 2503


## -- Part One -- ##

max_dist = max([distance(reindeer, duration) for reindeer in reindeers])
print(max_dist)


## -- Part Two -- ##

scores = [0] * len(reindeers)

for t in range(1, duration + 1):
    distances = [distance(reindeer, t) for reindeer in reindeers]
    leaders = [ind for ind, val in enumerate(distances) if val == max(distances)]
    for leader in leaders:
        scores[leader] += 1

print(max(scores))
