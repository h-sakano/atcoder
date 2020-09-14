import math

N = int(input())

X = math.ceil(N / 1.08)
N2 = math.floor(X * 1.08)

if N == N2:
    print(X)
else:
    print(':(')
