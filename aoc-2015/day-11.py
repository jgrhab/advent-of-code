input = "hepxcrrq"


def to_list(s: str) -> list[int]:
    return [ord(ch) - ord("a") for ch in s]


def to_str(ls: list[int]) -> str:
    return "".join([chr(x + ord("a")) for x in ls])


def increment(ls: list[int], val: int):
    carry = val
    for ind in reversed(range(len(ls))):
        sum = ls[ind] + carry
        carry = int(sum / 26)
        ls[ind] = sum % 26


def req1(ls: list[int]) -> bool:
    for ind in range(len(ls) - 3):
        if ls[ind + 1] == ls[ind] + 1 and ls[ind + 2] == ls[ind] + 2:
            return True
    return False


def req2(ls: list[int]) -> bool:
    return (
        (ord("i") - ord("a") not in ls)
        and (ord("o") - ord("a") not in ls)
        and (ord("l") - ord("a") not in ls)
    )


def req3(ls: list[int]) -> bool:
    pairs = set()
    for ind in range(len(ls) - 1):
        if ls[ind] == ls[ind + 1]:
            pairs.add(ls[ind])

    return len(pairs) >= 2


## -- Part One -- ##

ls = to_list(input)
while not (req1(ls) and req2(ls) and req3(ls)):
    increment(ls, 1)

print(to_str(ls))


## -- Part Two -- ##

increment(ls, 1)

while not (req1(ls) and req2(ls) and req3(ls)):
    increment(ls, 1)

print(to_str(ls))
