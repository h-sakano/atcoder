# -*- coding: utf-8 -*-
n, k = map(int, input().split())
s = input()
bl = []

left = -1
right = 0
for i in range(len(s)):
    if s[i] == '0':
        if left < 0:
            left = i
        if i == n - 1:
            right = n
            bl.append((left, right))
    else:
        if left >= 0:
            right = i
            bl.append((left, right))
            left = -1


def change(index, count, ts):
    if count >= k or index >= len(bl):
        max_seq = 0
        seq = 0
        for i in range(len(ts)):
            if ts[i] == '1':
                seq += 1
                if i == n - 1:
                    max_seq = max(max_seq, seq)
            else:
                max_seq = max(max_seq, seq)
                seq = 0
        return max_seq

    new_s = ts[:bl[index][0]] + '1' * \
        (bl[index][1] - bl[index][0]) + ts[bl[index][1]:]
    return max(change(index + 1, count + 1, new_s),
               change(index + 1, count, ts))


if k >= len(bl):
    print(len(s))
else:
    print(change(0, 0, s))
