# Parse target range
target_input = "target area: x=241..273, y=-97..-63" # real input
# target_input = "target area: x=20..30, y=-10..-5" # test

target_split = target_input.split(",")
x_index = target_split[0].index("x")
y_index = target_split[1].index("y")

x_range_str = target_split[0][x_index+2...]
y_range_str = target_split[1][y_index+2...]

x_range = x_range_str.split("..").map { |v| v.to_i }
y_range = y_range_str.split("..").map { |v| v.to_i }

x1 = x_range[0]
x2 = x_range[1]
y1 = y_range[0]
y2 = y_range[1]

# Get collidinng points and max y
CollidingValue = Struct.new(:vx, :vy, :max_y)

colliding = []

for y in y1..(y1.abs)
  for x in 1..(x2+1)
    vx = x
    vy = y

    px = 0
    py = 0

    current_max_y = 0
    for _ in 0..(2 * y1.abs + 2)
      px += vx
      py += vy

      vx -= (vx <=> 0)
      # vx = [vx - 1, 0].max
      vy -= 1

      current_max_y = [current_max_y, py].max

      if px >= x1 && px <= x2 && py >= y1 && py <= y2
        # collides
        colliding << CollidingValue.new(x, y, current_max_y)
        break
      elsif px > x2 || py < y1
        break
      end
    end
  end
end

# get max
max = colliding.reduce(CollidingValue.new(0, 0, 0)) do |max, cur|
  if cur.max_y > max.max_y
    cur
  else
    max
  end
end

puts "Part 1: #{max}"

puts "Part 2: #{colliding.count}"
