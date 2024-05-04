import hashlib

input = "bgvyzdsv"

## -- Part One -- ##

def first_digits(suffix: int, digits: int):
    m = hashlib.md5()
    s = input + str(suffix)
    m.update(s.encode())
    return m.hexdigest()[:digits]

suffix = 1
while first_digits(suffix, 5) != "00000":
    suffix += 1

print(suffix)


## -- Part Two -- ##

while first_digits(suffix, 6) != "000000":
    suffix += 1

print(suffix)
