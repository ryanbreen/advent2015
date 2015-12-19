
require 'png'

@canvas = PNG::Canvas.new 1000, 1000, PNG::Color::Black

@matrix = Array.new(1000)
for x in 0..999
  @matrix[x] = Array.new(1000, 0)
end

def handle_instruction(line)

  x1, y1, x2, y2 = line.match(/([\d]*),([\d]*)[^\d]*([\d]*),([\d]*)/).captures

  puts "#{x1}, #{y1}, #{x2}, #{y2} #{line}"

  x1 = x1.to_i
  x2 = x2.to_i
  y1 = y1.to_i
  y2 = y2.to_i

  puts "#{x1} #{y1} #{x2} #{y2} #{line}"

  if line.start_with? "turn on"
    puts "creating canvas with dimensions #{x2 - x1} #{y2 - y1}"

    # Create a new canvas the size of the new area
    new_region = PNG::Canvas.new ((x2 + 1) - x1), ((y2 + 1) - y1), PNG::Color::White

    # blt it onto the main canvas at the bottom left coords
    @canvas.composite new_region, x1, y1
  elsif line.start_with? "turn off"
    puts "creating canvas with dimensions #{x2 - x1} #{y2 - y1}"

    # Create a new canvas the size of the new area
    new_region = PNG::Canvas.new ((x2 + 1) - x1), ((y2 + 1) - y1), PNG::Color::Black

    # blt it onto the main canvas at the bottom left coords
    @canvas.composite new_region, x1, y1
  elsif line.start_with? "toggle"
    puts "Handling toggle of size #{x2 - x1} #{y2 - y1}"
    for x in x1..x2
      for y in y1..y2
        # Inverse the color
        if @canvas[x, y] == PNG::Color::Black
          @canvas[x, y] = PNG::Color::White
        else
          @canvas[x, y] = PNG::Color::Black
        end
      end
    end
  end
end

File.open("input.txt", "r") do |f|

  line_count = 0

  f.each_line do |line|
    handle_instruction line.strip
    line_count += 1
  end

  puts "Handled lines #{line_count}"

  total_white = 0

  # Walk the canvas counting lit lights
  @canvas.each do |x, y, color|
    total_white += 1 if color == PNG::Color::White
  end

=begin
  for x in 0..999
    for y in 0..999
      total_white += 1 if @matrix[x][y] == 1
    end
  end
=end
  puts "Total white: #{total_white}"

  png = PNG.new @canvas
  png.save 'grid.png'
  `open grid.png`
end
