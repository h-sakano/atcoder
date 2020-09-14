def gcd(a, b):
    while b != 0:
        a, b = b, a % b
    return a


def lcm(a, b):
    return a*b//gcd(a, b)


a, b, c, d = map(int, input().split())

first_c = c * (a // c + 1)
if a % c == 0:
    first_c -= c
last_c = c * (b // c)

first_d = d * (a // d + 1)
if a % d == 0:
    first_d -= d
last_d = d * (b // d)

cd = lcm(c, d)
first_cd = cd * (a // cd + 1)
if a % cd == 0:
    first_cd -= cd
last_cd = cd * (b // cd)

cnum = 0
dnum = 0
cdnum = 0

if first_c <= b:
    cnum = (last_c - first_c) // c + 1

if first_d <= b:
    dnum = (last_d - first_d) // d + 1

if first_cd <= b:
    cdnum = (last_cd - first_cd) // cd + 1

total = cnum + dnum - cdnum
print(b - a + 1 - total)
