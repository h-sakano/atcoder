K = int(input())
A, B = map(int, input().split())


if B // K - A // K > 0:
    print('OK')
elif B // K - A // K == 0:
    if A % K == 0 or B % K == 0:
        print('OK')
    else:
        print('NG')
else:
    print('NG')
