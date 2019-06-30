from collections import deque

N, M = map(int, input().split())
G = [[] for _ in range(3 * N)]

for _ in range(M):
    u, v = map(int, input().split())
    u, v = u - 1, v - 1

    # 頂点を3倍化して保存
    G[3 * u].append(3 * v + 1)
    G[3 * u + 1].append(3 * v + 2)
    G[3 * u + 2].append(3 * v)

S, T = map(int, input().split())
S, T = 3 * (S - 1), 3 * (T - 1)


def bfs():
    """幅優先探索"""
    cnt = 0
    q = deque()
    visited = [False for _ in range(3 * N)]
    q.append(S)
    visited[S] = True
    while len(q) > 0:
        new_q = deque()
        cnt += 1
        while(len(q) > 0):
            u = q.pop()
            for v in G[u]:
                if visited[v]:
                    continue
                if v == T:
                    return cnt // 3

                visited[v] = True
                new_q.append(v)
        q = new_q
    return -1


print(bfs())
