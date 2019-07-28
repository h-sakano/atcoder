N = int(input())
p = list(map(int, input().split()))
sorted_p = sorted(p)

cnt = 0

for i in range(N):
    if p[i] != sorted_p[i]:
        cnt += 1


if cnt == 0 or cnt == 2:
    print('YES')
else:
    print('NO')
