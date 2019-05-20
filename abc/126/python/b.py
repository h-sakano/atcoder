# -*- coding: utf-8 -*-
def canMonth(value):
    return 1 <= value and value <= 12


S = input()

F = int(S[:2])
B = int(S[2:])
fm = canMonth(F)
bm = canMonth(B)

if fm and bm:
    print('AMBIGUOUS')
elif not fm and not bm:
    print('NA')
elif fm:
    print('MMYY')
else:
    print('YYMM')
