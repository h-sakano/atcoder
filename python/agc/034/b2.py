# -*- coding: utf-8 -*-
qS = input()


def replace(s, count):
    fi = s.find('ABC')
    if fi >= 0:
        newS = s[:fi] + 'BCA' + s[fi+3:]
        return replace(newS, count + 1)
    else:
        return count


print(replace(qS, 0))
