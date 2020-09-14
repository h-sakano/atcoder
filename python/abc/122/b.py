S = input()
acgt = 'ACGT'

ans = 0
cnt = 0
for c in S:
    if acgt.find(c) >= 0:
        cnt += 1
    else:
        cnt = 0

    if cnt > ans:
        ans = cnt

print(ans)
