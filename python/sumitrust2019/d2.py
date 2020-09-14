N = int(input())
S = input()

ans = 0
for i in range(10):
    for j in range(10):
        for k in range(10):
            fi = S.find(str(i))
            if fi < 0:
                continue
            fj = S[fi+1:].find(str(j))
            if fj < 0:
                continue
            fk = S[fi+fj+2:].find(str(k))
            if fk < 0:
                continue
            ans += 1

print(ans)
