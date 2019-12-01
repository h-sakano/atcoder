X = int(input())

num = X // 100
amari = X % 100

if amari <= 5 * num:
    print(1)
else:
    print(0)
