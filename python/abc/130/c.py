w, h, x, y = map(int, input().split())

ans = w * h / 2

multi = 0
if x == w / 2 and y == h / 2:
    multi = 1

print(ans, multi)
