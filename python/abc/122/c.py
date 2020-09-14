n, q = map(int, input().split())
s = input()

for i in range(q):
    li, ri = map(int, input().split())
    t = s[li - 1:ri]
    a_flag = False
    cnt = 0
    for j in range(len(t)):
        if t[j] == 'A':
            a_flag = True
        elif t[j] == 'C':
            if a_flag:
                cnt += 1
                a_flag = False
        else:
            a_flag = False

    print(cnt)
