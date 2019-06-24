s = input()

prev_c = ""
ans = False
for c in s:
    if c == prev_c:
        ans = True
        break
    prev_c = c

if ans:
    print("Bad")
else:
    print("Good")
