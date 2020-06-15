def make_divisors(n):
    lower_divisors, upper_divisors = [], []
    i = 1
    while i*i <= n:
        if n % i == 0:
            lower_divisors.append(i)
            if i != n // i:
                upper_divisors.append(n//i)
        i += 1
    return lower_divisors + upper_divisors[::-1]


N = int(input())
A = list(map(int, input().split()))
setA = set(A)
dup = set([x for x in setA if A.count(x) > 1])

count = 0
for i in range(len(A)):
    flag = False
    if A[i] in dup:
        continue
    for div in make_divisors(A[i]):
        if div == A[i]:
            continue
        elif div in setA:
            flag = True
            break
    if not flag:
        count += 1

print(count)
