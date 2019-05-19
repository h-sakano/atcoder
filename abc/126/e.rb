N, M = gets.chomp.split(' ').map(&:to_i)

X = []
Y = []
Z = []

conn = []

M.times { |i|
  X[i], Y[i], Z[i] = gets.chomp.split(' ').map(&:to_i)
  included = false
  conn.map! { |c|
    if (c.include?(X[i]) || c.include?(Y[i]))
      included = true
      c = c | [X[i], Y[i]]
    end
    c
  }
  conn.push([X[i], Y[i]]) if !included
}

listed = conn.reduce([]) { |ret, c|
  ret | c
}

unlisted_length = N - listed.length

print(conn.length + unlisted_length)
