N, L = map(int, input().split())

a = []
min_v = float('inf')
min_i = 0
for i in range(N):
    value = L + i
    if abs(value) < min_v:
        min_v = abs(value)
        min_i = i
    a.append(value)

newa = a[:min_i] + a[min_i+1:]

print(sum(newa))
