import heapq
import copy

n, k = map(int, input().split())
v = list(map(int, input().split()))


def dfs(q, have, count, operate):
    if operate == 0:
        heapq.heappush(have, q[0])
        q = q[1:]
    elif operate == 1:
        heapq.heappush(have, q[-1])
        q = q[:-1]
    elif operate == 2:
        sub = heapq.heappop(have)
        q.insert(0, sub)
    elif operate == 3:
        sub = heapq.heappop(have)
        q.append(sub)

    count += 1
    if count >= k:
        return sum(have)

    a = -float('inf')
    b = -float('inf')
    c = -float('inf')
    d = -float('inf')
    e = dfs(q, have, count, -1)  # 何もしない

    if len(q) > 0:
        a = dfs(copy.deepcopy(q), copy.deepcopy(have), count, 0)
        b = dfs(copy.deepcopy(q), copy.deepcopy(have), count, 1)
    if len(have) > 0:
        c = dfs(copy.deepcopy(q), copy.deepcopy(have), count, 2)
        d = dfs(copy.deepcopy(q), copy.deepcopy(have), count, 3)

    return max(a, b, c, d, e)


print(dfs(v, [], -1, -1))
