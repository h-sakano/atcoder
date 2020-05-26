N, K = map(int, input().split())
A = list(map(int, input().split()))

history = {i+1: -1 for i in range(N)}
now = 1
history[now] = 0
hist = [now]
start = -1
end = -1
for i in range(K):
    now = A[now - 1]
    hist.append(now)
    if history[now] >= 0:
        start = history[now]
        end = i + 1
        break
    history[now] = i + 1

if start < 0 or end < 0:
    print(now)
else:
    index = (K - start) % (end - start)
    print(hist[start:end][index])
