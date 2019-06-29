s = input()

a = ''
b = ''
count_a = 0
count_b = 0
count_c = 0

for i in range(4):
    if s[i] == a:
        count_a += 1
    elif s[i] == b:
        count_b += 1
    elif a == '':
        a = s[i]
        count_a += 1
    elif b == '':
        b = s[i]
        count_b += 1
    else:
        count_c += 1

if count_c > 0 or count_a != 2 or count_b != 2:
    print('No')
else:
    print('Yes')
