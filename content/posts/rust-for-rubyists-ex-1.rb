def show_length(object)
  puts "Length of #{object.length}"
end

a = "Hello"
b = [1, 2, 3, 4]

show_length a
show_length b

c = b[0..1]

show_length c
