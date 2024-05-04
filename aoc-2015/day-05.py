import re

with open(0) as f:
    lines = f.read().splitlines()


## -- Part One -- ##


def cond_1(s: str) -> bool:
    counts = [s.count(ch) for ch in {"a", "e", "i", "o", "u"}]
    vowels = sum([x for x in counts if x > 0])
    return vowels >= 3


def cond_2(s: str) -> bool:
    for ind in range(len(s) - 1):
        if s[ind] == s[ind + 1]:
            return True

    return False


def cond_3(s: str) -> bool:
    return not ("ab" in s or "cd" in s or "pq" in s or "xy" in s)


total = sum([cond_1(s) and cond_2(s) and cond_3(s) for s in lines])
print(total)


## -- Part Two -- ##


def cond_4(s: str) -> bool:
    for ind in range(len(s) - 3):
        if s[ind : ind + 2] in s[ind + 2 :]:
            return True
    return False


def cond_5(s: str) -> bool:
    return re.search("(?P<letter>\w)\w(?P=letter)", s) is not None


total = sum([cond_4(s) and cond_5(s) for s in lines])
print(total)
