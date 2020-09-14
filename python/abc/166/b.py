N, K = map(int, input().split())

ans = {i + 1 for i in range(N)}
for i in range(K):
    d = int(input())
    A = map(int, input().split())
    for a in A:
        ans.discard(a)

print(len(ans))
