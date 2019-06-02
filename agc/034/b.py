# -*- coding: utf-8 -*-
import re

qS = input()

counts = {}


def replace(s, count):
    if counts.get(s, -1) >= 0:
        return counts[s]
    else:
        matches = re.finditer('ABC', s)
        max_count = count
        for m in matches:
            m.span()
            max_count = max(max_count, replace(
                s[:m.span()[0]] + 'BCA' + s[m.span()[1]:], count + 1))
        counts[s] = max_count
        return counts[s]


print(replace(qS, 0))
