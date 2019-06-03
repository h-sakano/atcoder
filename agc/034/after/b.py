# -*- coding: utf-8 -*-
s = input()
s = s.replace('BC', 'D')

a_num = 0
ans = 0
for c in s:
    if c == 'A':
        a_num += 1
    elif c == 'D':
        ans += a_num
    else:
        a_num = 0

print(ans)
