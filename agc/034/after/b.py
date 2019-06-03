# -*- coding: utf-8 -*-
s = input()

a_num = 0
ans = 0
i = 0
while i < len(s):
    if s[i] == 'A':
        a_num += 1
        i += 1
    elif s[i:i+2] == 'BC':
        ans += a_num
        i += 2
    else:
        a_num = 0
        i += 1

print(ans)
