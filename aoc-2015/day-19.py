import re

with open(0) as f:
    lines = f.read().splitlines()

CLEAN = {
    "Rn": "(",
    "Ar": ")",
    "Y": ",",
    "Ca": "K",
    "Al": "A",
    "Si": "S",
    "Ti": "I",
    "Th": "T",
    "Mg": "M",
}


def clean_str(s: str) -> str:
    for old, new in CLEAN.items():
        s = s.replace(old, new)

    return s


molecule = clean_str(lines[-1])

reps: dict[str, set[str]] = dict()
for line in lines[:-2]:
    split = clean_str(line).split()

    if split[0] not in reps:
        reps[split[0]] = set()

    reps[split[0]].add(split[2])


# -- Part One -- #

one_rep = set()

for old in reps.keys():
    for new in reps[old]:
        matches = (m.start() for m in re.finditer(f"{old}", molecule))
        one_rep.update({molecule[:m] + new + molecule[m + len(old) :] for m in matches})

print(len(one_rep))


# -- Part Two -- #

# The replacements for a single character are: X -> YZ, Y(A), Y(A,B), Y(A,B,C)
# They increase the length of the string by 1, 3, 5, and 7 resp.
# Denote their number by x1, x3, x5, x7.
# We have: 1 + (1 * x1) + (3 * x3) + (5 * x5) + (7 * x7) = len(molecule)

x7 = len(re.findall("\w\([\w\(\),]+,[\w\(\),]+,[\w\(\),]+\)", molecule))

# Cannot find x5 using regex because of the nested patterns.
# Instead, note that:
# #{ pairs of parentheses } = x3 + x5 + x7
# #{ comma } = x5 + 2 * x7

commas = molecule.count(",")
par_pairs = molecule.count("(")

x5 = commas - 2 * x7
x3 = par_pairs - x7 - x5

# Using the first equation, we get
x1 = len(molecule) - (7 * x7) - (5 * x5) - (3 * x3) - 1

# The total is
repl_count = x1 + x3 + x5 + x7
print(repl_count)
