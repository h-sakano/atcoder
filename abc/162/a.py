N = input()
lucky = False
for s in N:
    if s == '7':
        lucky = True
        break

if lucky:
    print('Yes')
else:
    print('No')
