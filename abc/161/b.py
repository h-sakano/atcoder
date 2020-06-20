import math

N, M = map(int, input().split())
A = list(map(int, input().split()))
A.sort(reverse=True)
total = sum(A)

if A[M - 1] < math.ceil(total / (4 * M)):
    print('No')
else:
    print('Yes')
