
def needed_sq_feet(dimensions)

  slack = dimensions[0] * dimensions[1]

  slack +
    (2 * (dimensions[0] * dimensions[1])) +
    (2 * (dimensions[0] * dimensions[2])) +
    (2 * (dimensions[1] * dimensions[2]))

end

File.open("input.txt", "r") do |f|

  total_sf = 0;

  f.each_line do |line|
    sides = line.split('x').map{|a| a.to_i }.sort
    total_sf += needed_sq_feet sides
  end

  puts total_sf
end