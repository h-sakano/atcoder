N = int(input())
d = list(map(int, input().split()))
d.sort()

abc = 0
arc = N

ans = 0
for k in range(1, 10 ** 5 + 1):
    while True:
        if abc >= N:
            break
        elif d[abc] < k:
            abc += 1
            arc -= 1
        else:
            break
    if abc == arc:
        ans += 1
    elif abc > arc:
        break

print(ans)
