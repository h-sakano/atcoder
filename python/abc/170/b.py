X, Y = map(int, input().split())

if Y % 2 != 0:
    print('No')
else:
    if 2 * X <= Y and Y <= 4 * X:
        print('Yes')
    else:
        print('No')
