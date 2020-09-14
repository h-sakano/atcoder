import math
X = int(input())

now = 100
count = 0
while True:
    count += 1
    now = math.floor(now * 1.01)
    if now >= X:
        break

print(count)
