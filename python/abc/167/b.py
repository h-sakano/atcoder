A, B, C, K = map(int, input().split())

nokori = K
num_1 = min(A, nokori)
nokori -= num_1
ans = num_1

nokori -= min(B, nokori)

num_minus = min(C, nokori)
ans -= num_minus

print(ans)
