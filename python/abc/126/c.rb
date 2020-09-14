N, K = gets.chomp.split(' ').map(&:to_i);

ret = 0
last = N
if N >= K
  last = K - 1
  ret += (N - K + 1) / N.to_f if N >= K
end
(1..last).each { |p|
  log = 0
  x = p
  while (x < K) do
    x *= 2
    log += 1
  end
  ret += (1 / N.to_f) * (0.5 ** log)
}
print(ret)
