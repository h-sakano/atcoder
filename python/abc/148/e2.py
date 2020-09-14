N = int(input())

if N % 2 != 0:
    print(0)
else:
    denominator = 10
    ans = 0
    while denominator <= N:
        ans += N // denominator
        denominator *= 5

    print(ans)
