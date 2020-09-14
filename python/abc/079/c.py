ABCD = input()


def dfs(total=0, level=0, ans=0):
    level += 1
    if level >= 4:
        if total == 7:
            return ans
        return -1
    plus_ans = dfs(total + int(ABCD[level]), level, ans + (1 << (level - 1)))
    if plus_ans >= 0:
        return plus_ans
    minus_ans = dfs(total - int(ABCD[level]), level, ans)
    if minus_ans:
        return minus_ans
    return -1


ans = dfs(int(ABCD[0]), 0, 0)
op1 = "+" if ans & 1 != 0 else "-"
op2 = "+" if ans & (1 << 1) != 0 else "-"
op3 = "+" if ans & (1 << 2) != 0 else "-"
print(ABCD[0] + op1 + ABCD[1] + op2 + ABCD[2] + op3 + ABCD[3] + "=7")
