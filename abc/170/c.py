X, N = map(int, input().split())
P = list(map(int, input().split()))

li = []
for i in range(0, 102):
    if i not in P:
        li.append(i)

ans = -1
minimum_num = 200

for n in li:
    num = abs(n - X)
    if num < minimum_num:
        minimum_num = num
        ans = n

print(ans)
