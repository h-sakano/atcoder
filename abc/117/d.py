N, K = map(int, input().split())
A = list(map(int, input().split()))
BIT_NUM = K.bit_length()

X = 0
flag = False
for i in reversed(range(BIT_NUM)):
    cnt = 0
    for a in A:
        if a >> i & 1:
            cnt += 1
    if flag:
        if N - cnt > cnt:
            X |= 1 << i
        continue
    kbit = (K >> i) & 1
    if N - cnt > cnt and kbit:
        # Kのbitが立っていないところでXのビットを立てると、XがKより大きくなってしまう
        X |= 1 << i
    else:
        if kbit:
            # 以降はKとの大小比較を考えなくても良い
            flag = True

print(sum([X ^ a for a in A]))
