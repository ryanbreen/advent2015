
require 'png'

@canvas = PNG::Canvas.new 1000, 1000, PNG::Color::Black

def handle_instruction(line)

  x1, y1, x2, y2 = line.match(/([\d]*),([\d]*)[^\d]*([\d]*),([\d]*)/).captures

  puts "#{x1}, #{y1}, #{x2}, #{y2} #{line}"

  x1 = x1.to_i
  x2 = x2.to_i
  y1 = y1.to_i
  y2 = y2.to_i

  puts "#{x1} #{y1} #{x2} #{y2} #{line}"

  if line.start_with? "turn"
    is_on = line.include? "on"
    puts "Handling turn #{is_on} size #{x2 - x1} #{y2 - y1}"
    for x in x1..x2
      for y in y1..y2
        if is_on
          @canvas[x, y] = PNG::Color.new(@canvas[x, y].r() + 1, 0, 0)
        else
          new_color = 0
          new_color = @canvas[x, y].r() - 1 if @canvas[x, y].r() != 0
          @canvas[x, y] = PNG::Color.new(new_color, 0, 0)
        end
      end
    end
  elsif line.start_with? "toggle"
    puts "Handling toggle of size #{x2 - x1} #{y2 - y1}"
    for x in x1..x2
      for y in y1..y2
        @canvas[x, y] = PNG::Color.new(@canvas[x, y].r() + 2, 0, 0)
      end
    end
  end
end

File.open("input.txt", "r") do |f|

  f.each_line do |line|
    handle_instruction line.strip
  end

  total_bright = 0

  # Walk the canvas counting lit lights
  @canvas.each do |x, y, color|
    total_bright += color.r()
  end

  puts "Total color: #{total_bright}"

  png = PNG.new @canvas
  png.save 'grid.png'
  `open grid.png`
end
