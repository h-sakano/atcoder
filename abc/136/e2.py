N, K = map(int, input().split())
A = list(map(int, input().split()))

sumA = sum(A)
divisors = []
for i in range(1, sumA + 1):
    if i * i > sumA:
        break
    if sumA % i != 0:
        continue
    divisors.append(i)
    if sumA // i != i:
        divisors.append(sumA // i)

divisors.sort(reverse=True)

ans = 0
for d in divisors:
    costs = [a % d for a in A]
    costs.sort(reverse=True)
    plus = sum(costs)
    minus = 0
    for j in range(N):
        if plus == minus:
            break
        else:
            plus -= costs[j]
            minus += d - costs[j]

    if plus <= K:
        print(d)
        break
