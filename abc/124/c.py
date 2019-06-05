# -*- coding: utf-8 -*-
s = input()
b = []

for i in range(len(s)):
    if s[i] == '0':
        b.append(False)
    else:
        b.append(True)

switch = False
c1 = 0
for i in range(len(b)):
    if b[i] != switch:
        c1 += 1
    switch = not switch

switch = True
c2 = 0
for i in range(len(b)):
    if b[i] != switch:
        c2 += 1
    switch = not switch

print(min(c1, c2))
