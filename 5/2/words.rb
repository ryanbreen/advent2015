
def nice?(word)

  n_minus_2_char = nil
  last_char = nil

  doubles = {}

  n_minus_2_met = false
  double_met = false

  i = 0

  word.each_byte do |byte|
    char = byte.chr

    i+=1

    if last_char != nil
      pair = "#{last_char}#{char}"
      double_met = true if word[i..word.length].include? pair
    end

    n_minus_2_met = true if n_minus_2_char == char

    n_minus_2_char = last_char
    last_char = char
  end

  puts "#{word} #{double_met} #{n_minus_2_met}"

  double_met && n_minus_2_met

end

File.open("input.txt", "r") do |f|

  nice_count = 0;

  f.each_line do |line|
    nice_count += 1 if nice? line.strip
  end

  puts nice_count
end