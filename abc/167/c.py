N, M, X = map(int, input().split())
books = []
for i in range(N):
    book = list(map(int, input().split()))
    books.append(book)

min_cost = -1
for i in range(2 ** N):
    cost = 0
    skills = [0] * M
    for j in range(N):
        if i & (2 ** j) != 0:
            cost += books[j][0]
            for s in range(M):
                skills[s] += books[j][s + 1]
    if min(skills) >= X and (min_cost < 0 or cost < min_cost):
        min_cost = cost

print(min_cost)
