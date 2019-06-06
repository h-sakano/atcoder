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

ans = 0
left = 0
right = 0
tmp = 0

for i in range(0, len(nums), 2):
    next_left = i
    next_right = min(i + 2 * k + 1, len(nums))

    while (left < next_left):
        tmp -= nums[left]
        left += 1

    while (right < next_right):
        tmp += nums[right]
        right += 1

    ans = max(ans, tmp)

print(ans)
