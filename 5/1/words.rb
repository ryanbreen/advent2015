
@vowels = {
  'a' => true,
  'e' => true,
  'i' => true,
  'o' => true,
  'u' => true,
}

def nice?(word)

  last_char = nil

  double_char_met = false
  vowel_count = 0;

  word.each_byte do |byte|
    char = byte.chr
    return false if last_char == 'a' && char == 'b' 
    return false if last_char == 'c' && char == 'd'
    return false if last_char == 'p' && char == 'q'
    return false if last_char == 'x' && char == 'y'

    double_char_met = true if char == last_char

    vowel_count += 1 if @vowels[char]

    last_char = char
  end

  puts "#{word} #{vowel_count > 3 && double_char_met}"

  vowel_count >= 3 && double_char_met

end

File.open("input.txt", "r") do |f|

  nice_count = 0;

  f.each_line do |line|
    nice_count += 1 if nice? line.strip
  end

  puts nice_count
end