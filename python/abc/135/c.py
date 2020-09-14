N = int(input())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

ans = 0
for i in range(N):
    ans += min(A[i], B[i])
    num = max(0, B[i] - A[i])
    battle_num = min(A[i + 1], num)
    ans += battle_num
    A[i + 1] -= battle_num

print(ans)
