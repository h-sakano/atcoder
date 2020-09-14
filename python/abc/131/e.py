N, K = map(int, input().split())

min_num = N - 2

a = [0, 0]
add = 1
for i in range(2, N):
    a.append(a[i - 1] + add)
    add += 1

max_num = a[N - 1]

if max_num < K:
    print(-1)
else:
    yobun = K - min_num
    if yobun >= 0:
        print(N - 1)
        for i in range(2, N + 1):
            if i <= 2:
                print(i - 1, i)
            else:
                if yobun > 0:
                    print(2, i)
                    yobun -= 1
                else:
                    print(i - 1, i)
    else:
        print(N - 1 + abs(yobun))
        for i in range(2, N + 1):
            print(i - 1, i)
        for i in range(abs(yobun)):
            print(i + 1, i + 3)
