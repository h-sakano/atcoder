class UnionFind:
    def __init__(self, n):
        self.parent = [i for i in range(n + 1)]
        self.rank = [0] * (n + 1)
        self.size = [1] * (n + 1)

    # 根の検索
    def find(self, x):
        if self.parent[x] == x:
            return x
        else:
            self.parent[x] = self.find(self.parent[x])
            return self.parent[x]

    # 併合
    def union(self, x, y):
        x, y = self.find(x), self.find(y)
        if x == y:
            return
        if self.rank[x] < self.rank[y]:
            self.parent[x] = y
            self.size[y] += self.size[x]
        else:
            self.parent[y] = x
            self.size[x] += self.size[y]
            if self.rank[x] == self.rank[y]:
                self.rank[x] += 1

    # 同じ集合に属するか判定
    def is_same(self, x, y):
        return self.find(x) == self.find(y)

    # グループのサイズ
    def get_size(self, x):
        return self.size[self.find(x)]


N, M = map(int, input().split())
bridges = []

for i in range(M):
    a, b = map(int, input().split())
    bridges.append((a, b,))
bridges = list(reversed(bridges))

ans = [N * (N - 1) // 2]
uf = UnionFind(N)
for i in range(M):
    a, b = bridges[i]
    tmp = 0
    if uf.is_same(a, b):
        tmp = ans[i]
    else:
        tmp = ans[i] - uf.get_size(a) * uf.get_size(b)
        uf.union(a, b)

    ans.append(tmp)

for i in range(M - 1, -1, -1):
    print(ans[i])
