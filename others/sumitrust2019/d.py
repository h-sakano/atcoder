N = int(input())
S = input()

ans = 0
kinds = [set() for _ in range(N + 1)]
history = set()

for i in range(N - 1, -1, -1):
    kinds[i] = kinds[i + 1].union(S[i])

for i in range(N - 2):
    for j in range(i + 1, N - 1):
        if not (S[i], S[j],) in history:
            ans += len(kinds[j+1])
            history.add((S[i], S[j],))

print(ans)
