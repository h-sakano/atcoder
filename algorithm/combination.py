def comb(n, k, m):
    """power_funcを用いて(nCk) mod m を求める"""
    from math import factorial
    if n < 0 or k < 0 or n < k:
        return 0
    if n == 0 or k == 0:
        return 1
    a = factorial(n) % m
    b = factorial(k) % m
    c = factorial(n-k) % m
    return (a * power_func(b, m - 2, m) * power_func(c, m - 2, m)) % m


def power_func(a, b, m):
    """a^b mod m を求める"""
    if b == 0:
        return 1
    if b % 2 == 0:
        d = power_func(a, b // 2, m)
        return d * d % m
    if b % 2 == 1:
        return (a * power_func(a, b - 1, m)) % m
