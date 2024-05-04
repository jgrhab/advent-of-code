import re

input = "3113322113"


def look_and_say(s: str) -> str:
    matches = re.findall("((?P<digit>\d)(?P=digit)*)", s)

    result = ""
    for match in matches:
        result += str(len(match[0])) + match[1]

    return result


## -- Part One -- ##

s = input
for _ in range(40):
    s = look_and_say(s)

print(len(s))


## -- Part Two -- ##

for _ in range(10):
    s = look_and_say(s)

print(len(s))
