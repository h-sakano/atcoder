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
    # 一旦全部マイナス方向に調整することにしたら総和はXの倍数になる
    # どれか一つ符号を逆転すると総和がx減るので、総和/d番目が境目になる
    # A = [8, 20]
    # d = 7
    # costs = [1, 6]
    # 20をプラス方向に調整することにすると、マイナス方向の調整は6減ってプラス方向に(7 - 1)増える
    # 1個をマイナス方向をプラス方向に変更すると、マイナス方向のコストの総和の差とプラス方向のコストの総和の差がdだけ詰まる
    # よって、最初全部マイナス方向に調整したときのコストの総和7を7で割ったところが境目
    # プラス方向にはcostが大きい方から変更したい(調整の量が少なくてすむ)ので、最初にソートしておく
    costs = [a % d for a in A]
    costs.sort()
    if sum(costs[:N - sum(costs) // d]) <= K:
        print(d)
        break
