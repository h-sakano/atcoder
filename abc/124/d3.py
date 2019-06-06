# -*- coding: utf-8 -*-
n, k = map(int, input().split())
s = input()
nums = []
now = 1

cnt = 0
for i in range(n):
    if s[i] == str(int('0') + now):
        cnt += 1
    else:
        nums.append(cnt)
        now = 1 - now
        cnt = 1

if cnt != 0:
    nums.append(cnt)

t = [0]
for i in range(len(nums)):
    t.append(t[i] + nums[i])

ans = 0
for i in range(0, len(nums), 2):
    left = i
    right = min(i + 2 * k + 1, len(nums))

    ans = max(ans, t[right] - t[left])

print(ans)
