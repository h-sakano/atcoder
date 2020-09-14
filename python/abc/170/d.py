N = int(input())
A = list(map(int, input().split()))

A.sort()

count = 0
for i in range(len(A)):
    if A[i] in A[:i]:
        continue
    if A[i] in A[i+1:]:
        continue
    flag = False
    for j in range(0, i):
        if (A[i] % A[j] == 0):
            flag = True
            break
    if not flag:
        count += 1

print(count)
