N = int(input())
X_dumy = N
out = ''
while X_dumy > 0:
    out = chr(ord('a') + (X_dumy - 1) % 26) + out
    X_dumy = int((X_dumy - 1) / 26)

print(out)
