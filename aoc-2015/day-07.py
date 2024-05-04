from typing import Dict

with open(0) as f:
    lines = f.read().splitlines()


instructions = [line.split() for line in lines]


def execute(ins: list[str]) -> bool:
    global wires

    if ins[1] == "->" and ins[0].isdigit():
        wires[ins[-1]] = int(ins[0])
        return True
    elif ins[1] == "->" and ins[0] in wires:
        wires[ins[-1]] = wires[ins[0]]
        return True
    elif ins[0] == "NOT" and ins[1] in wires:
        # need XOR as the number of bits is not fixed
        wires[ins[-1]] = wires[ins[1]] ^ 0b1111111111111111
        return True
    elif ins[1] == "AND" and ins[0] in wires and ins[2] in wires:
        wires[ins[-1]] = wires[ins[0]] & wires[ins[2]]
        return True
    elif ins[1] == "AND" and ins[0].isdigit() and ins[2] in wires:
        # there are some "1 and x -> z"
        wires[ins[-1]] = int(ins[0]) & wires[ins[2]]
        return True
    elif ins[1] == "OR" and ins[0] in wires and ins[2] in wires:
        wires[ins[-1]] = wires[ins[0]] | wires[ins[2]]
        return True
    elif ins[1] == "LSHIFT" and ins[0] in wires:
        wires[ins[-1]] = wires[ins[0]] << int(ins[2])
        return True
    elif ins[1] == "RSHIFT" and ins[0] in wires:
        wires[ins[-1]] = wires[ins[0]] >> int(ins[2])
        return True

    return False


## -- Part One -- ##

wires: Dict[str, int] = dict()
remaining_ins = instructions.copy()

while remaining_ins:
    for ind in reversed(range(len(remaining_ins))):
        if execute(remaining_ins[ind]):
            remaining_ins.pop(ind)

signal_a = wires["a"]
print(signal_a)


## -- Part Two -- ##

wires: Dict[str, int] = dict()

# Replace the instruction that assigns to "b"
ins_b = [ind for ind, ins in enumerate(instructions) if ins[-1] == "b"]
instructions.pop(ins_b[0])
instructions.append([f"{signal_a}", "->", "b"])

while instructions:
    for ind in reversed(range(len(instructions))):
        if execute(instructions[ind]):
            instructions.pop(ind)

print(wires["a"])
