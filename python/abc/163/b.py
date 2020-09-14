N, M = map(int, input().split())
A = list(map(int, input().split()))

days = sum(A)
holidays = N - days

if holidays >= 0:
    print(holidays)
else:
    print(-1)
