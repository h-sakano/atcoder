import heapq

x, y, z, k = map(int, input().split())
a = list(map(int, input().split()))
b = list(map(int, input().split()))
c = list(map(int, input().split()))
a.sort(reverse=True)
b.sort(reverse=True)
c.sort(reverse=True)

cnt = 0
q = []
arg = (0, 0, 0)
visited = {}
visited[arg] = True
heapq.heappush(q, (-(a[arg[0]] + b[arg[1]] + c[arg[2]]),) + arg)

while cnt < k:
    score, ia, ib, ic = heapq.heappop(q)
    print(-score)
    cnt += 1
    arg_a = (ia + 1, ib, ic)
    arg_b = (ia, ib + 1, ic)
    arg_c = (ia, ib, ic + 1)
    if arg_a[0] <= x - 1 and arg_a not in visited:
        heapq.heappush(
            q, (-(a[arg_a[0]] + b[arg_a[1]] + c[arg_a[2]]),) + arg_a)
        visited[arg_a] = True
    if arg_b[1] <= y - 1 and arg_b not in visited:
        heapq.heappush(
            q, (-(a[arg_b[0]] + b[arg_b[1]] + c[arg_b[2]]),) + arg_b)
        visited[arg_b] = True
    if arg_c[2] <= z - 1 and arg_c not in visited:
        heapq.heappush(
            q, (-(a[arg_c[0]] + b[arg_c[1]] + c[arg_c[2]]),) + arg_c)
        visited[arg_c] = True
