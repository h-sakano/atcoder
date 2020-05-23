import math

A, B, H, M = map(int, input().split())

theta = 360 * abs(M / 60 - (H / 12 + M / (60 * 12)))
if theta > 180:
    theta = 360 - theta

print(math.sqrt(A * A + B * B - 2 * A * B * math.cos(math.radians(theta))))
