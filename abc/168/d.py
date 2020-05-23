N, M = map(int, input().split())
bridges = [[] for i in range(N)]

for i in range(M):
    a, b = map(int, input().split())
    bridges[a - 1].append(b - 1)
    bridges[b - 1].append(a - 1)

ans = [-1] * N
q = [0]
while len(q) > 0:
    nq = []
    for t in q:
        for e in bridges[t]:
            if ans[e] < 0:
                nq.append(e)
                ans[e] = t
    q = nq

if -1 in ans:
    print('No')
else:
    print("Yes")
    for a in ans[1:]:
      print(a + 1)
