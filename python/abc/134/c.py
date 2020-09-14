N = int(input())
A = []
for i in range(N):
    A.append(int(input()))

sortA = sorted(A)
maxA = sortA[-1]
next_maxA = sortA[-2]

for i in range(N):
    if A[i] == maxA:
        print(next_maxA)
    else:
        print(maxA)
