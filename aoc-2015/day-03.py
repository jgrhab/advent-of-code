with open(0) as f:
    input = f.read().rstrip("\n")


def visit(input: str) -> set((int, int)):
    x, y = 0, 0
    visited = {(x, y)}

    for ch in input:
        match ch:
            case "^":
                x += 1
            case "v":
                x -= 1
            case "<":
                y -= 1
            case ">":
                y += 1
            case _:
                raise RuntimeError()

        visited.add((x, y))

    return visited


## -- Part One -- ##

visited = visit(input)

print(len(visited))


## -- Part Two -- ##

input_human = [ch for ind, ch in enumerate(input) if ind % 2 == 0]
input_robot = [ch for ind, ch in enumerate(input) if ind % 2 == 1]

visited_human = visit(input_human)
visited_robot = visit(input_robot)

visited = visited_human | visited_robot

print(len(visited))
