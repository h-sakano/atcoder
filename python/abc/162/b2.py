N = int(input())


def sum(n):
    return (n + 1) * n // 2


print(sum(N) - sum(N // 3) * 3 - sum(N // 5) * 5 + sum(N // 15) * 15)
