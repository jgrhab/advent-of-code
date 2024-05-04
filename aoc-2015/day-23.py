with open(0) as f:
    lines = f.read().splitlines()

instructions = [[s.strip(",") for s in line.split()] for line in lines]


def execute(pos: int) -> int:
    global registers
    ins = instructions[pos]

    match ins[0]:
        case "hlf":
            registers[ins[1]] = int(registers[ins[1]] / 2)

        case "tpl":
            registers[ins[1]] *= 3

        case "inc":
            registers[ins[1]] += 1

        case "jmp":
            pos += int(ins[1]) - 1

        case "jie":
            if registers[ins[1]] % 2 == 0:
                pos += int(ins[2]) - 1

        case "jio":
            if registers[ins[1]] == 1:
                pos += int(ins[2]) - 1

        case _:
            raise RuntimeError()

    pos += 1

    return pos


# -- Part One -- #

registers = {"a": 0, "b": 0}
pos = 0

while 0 <= pos and pos < len(instructions):
    pos = execute(pos)

print(registers["b"])


# -- Part Two -- #

registers = {"a": 1, "b": 0}
pos = 0


while 0 <= pos and pos < len(instructions):
    pos = execute(pos)

print(registers["b"])
