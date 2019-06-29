N, K = map(int, input().split())
MOD = 10 ** 9 + 7


def comb(n, k, p):
    """power_funcを用いて(nCk) mod p を求める"""
    from math import factorial
    if n < 0 or k < 0 or n < k:
        return 0
    if n == 0 or k == 0:
        return 1
    a = factorial(n) % p
    b = factorial(k) % p
    c = factorial(n-k) % p
    return (a*power_func(b, p-2, p)*power_func(c, p-2, p)) % p


def power_func(a, b, p):
    """a^b mod p を求める"""
    if b == 0:
        return 1
    if b % 2 == 0:
        d = power_func(a, b//2, p)
        return d*d % p
    if b % 2 == 1:
        return (a*power_func(a, b-1, p)) % p


for i in range(1, K + 1):
    print(comb(N - K + 1, i, MOD) * comb(K - 1, i - 1, MOD) % MOD)
