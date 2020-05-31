import math
from decimal import Decimal

A, B = input().split()
a = int(A)
b = Decimal(B)
print(math.floor(a * b))
