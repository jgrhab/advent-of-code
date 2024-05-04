import numpy as np  # type: ignore

with open(0) as f:
    lines = f.read().splitlines()


class Instruction:
    def __init__(self, s: str):
        split = s.split()

        # Remove the "turn"
        if len(split) == 5:
            split = split[1:]

        self.action = split[0]
        self.start = list(map(int, split[1].split(",")))
        self.end = list(map(int, split[-1].split(",")))


instructions = [Instruction(line) for line in lines]


## -- Part One -- ##

brightness = np.full((1000, 1000), False)

for ins in instructions:
    x0, y0 = ins.start[0], ins.start[1]
    x1, y1 = ins.end[0] + 1, ins.end[1] + 1
    match ins.action:
        case "on":
            brightness[x0:x1, y0:y1] = True
        case "off":
            brightness[x0:x1, y0:y1] = False
        case "toggle":
            brightness[x0:x1, y0:y1] = ~brightness[x0:x1, y0:y1]
        case _:
            raise RuntimeError()

print(np.sum(brightness))


## -- Part Two -- ##

brightness = np.full((1000, 1000), 0)
zeros = np.zeros((1000, 1000), dtype=np.int32)

for ins in instructions:
    x0, y0 = ins.start[0], ins.start[1]
    x1, y1 = ins.end[0] + 1, ins.end[1] + 1
    match ins.action:
        case "on":
            brightness[x0:x1, y0:y1] += 1
        case "off":
            brightness[x0:x1, y0:y1] -= 1
            brightness = np.maximum(brightness, zeros)
        case "toggle":
            brightness[x0:x1, y0:y1] += 2
        case _:
            raise RuntimeError()

print(np.sum(brightness))
