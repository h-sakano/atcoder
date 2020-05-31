import math
N = int(input())
start1 = 0
end1 = 0
start2 = 0
end2 = 0
start1, end1 = map(int, input().split())
start2, end2 = map(int, input().split())
medians = [(start1, end1), (start2, end2)]
for i in range(N):
    start, end = map(int, input().split())
    if math.max(medians[0][0], medians[1][0]) < start and start < math.min(medians[0][1], medians[1][1]):
        if (medians[0][0] < medians[1][0]):
            medians[0][0] = start
        else:
            medians[1][0] = start


if N % 2 == 0:
    pass
else:
    pass
