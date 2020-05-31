N = int(input())
A = list(map(int, input().split()))
ans = 1
dekai = False
for a in A:
    ans *= a
    if ans > (10 ** 18):
        dekai = True
        break

if 0 in A:
    print(0)
elif dekai:
    print(-1)
else:
    print(ans)
