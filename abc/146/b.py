N = int(input())
S = input()

N %= 26

for s in S:
    o = ord(s) + N
    if o > ord('Z'):
        o = ord('A') + (o - ord('Z')) - 1
    print(chr(o), end="")
print()
