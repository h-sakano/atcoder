S = gets.chomp

first = S[0..1].to_i
second = S[2..3].to_i

ft = false
ft = true if (1 <= first  && first <= 12)

st = false
st = true if (1 <= second  && second <= 12)

if (ft && st)
  print('AMBIGUOUS')
elsif (!ft && !st)
  print('NA')
elsif ft
  print('MMYY')
else
  print('YYMM')
end
