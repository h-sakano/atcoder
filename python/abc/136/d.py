S = input() + 'R'
N = len(S)
ans = [0 for _ in range(N - 1)]
switch = 'R'
countR = 0
countL = 0
point = 0
for i in range(N):
    if S[i] == switch:
        if switch == 'R':
            countR += 1
        else:
            countL += 1
    else:
        if switch == 'L':
            switch = 'R'
            count = (countR + countL)
            count_half = count // 2
            if count % 2 == 0:
                ans[point - 1] = count_half
                ans[point] = count_half
            else:
                if countR % 2 == 0:
                    ans[point - 1] = count_half
                    ans[point] = count_half + 1
                else:
                    ans[point - 1] = count_half + 1
                    ans[point] = count_half
            countR = 1
            countL = 0
        else:
            switch = 'L'
            countL = 1
            point = i

print(' '.join(map(str, ans)))
