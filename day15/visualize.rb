require_relative "data.rb"
require 'ruby2d'

# scale factor
SCALE_F = 8

set width: 100 * SCALE_F, height: 100 * SCALE_F

lines = Data::INPUT.split "\n"

t = Thread.new {
  # squares = []
  # text = []
  prev_was_red = true
  # x = 0
  # y = 0
  for y in 0...100
    for x in 0...100
      color = ''
      if prev_was_red
        color = 'black'
      else
        color = 'red'
      end
      prev_was_red = !prev_was_red
      # puts "#{color}, #{prev_was_red}, (#{x}, #{y})"
    
      Square.new(
        x: x * SCALE_F, y: y * SCALE_F,
        size: 1 * SCALE_F,
        color: color,
        z: 1
      )

      # x += 1
  
      # if x == 100
      #   # x = 0
      #   # y += 1
      #   prev_was_red = !prev_was_red
      # end
    end
    prev_was_red = !prev_was_red
  end
}

# t3 = Thread.new {
#   # mutex = Mutex.new
#   gy = 0
    
#   threads = []
  
#   for _line in lines
#     threads << Thread.new(gy, _line) { |y, line|
#       # mutex.lock
#       # y = gy
#       # mutex.unlock
#       x = 0
#       # line = _line
    
#       for char in line.chars
#         Text.new(
#           char,
#           x: x * SCALE_F, y: y * SCALE_F,
#           size: 1 * SCALE_F,
#           color: 'white',
#           z: 3
#         )
#         x += 1
#       end
#     }
  
#     # mutex.lock
#     gy += 1
#     # mutex.unlock
  
#     # for char in line.chars
#     #   threads << Thread.new{

    
#     #     Text.new(
#     #       char,
#     #       x: x * SCALE_F, y: y * SCALE_F,
#     #       size: 1 * SCALE_F,
#     #       color: 'white',
#     #       z: 3
#     #     )
#     #     x += 1
  
#     #     if x == 100
#     #       x = 0
#     #       y += 1
#     #     end
#     #   }
#     # end
#     # y += 1
  
#     # for thread in threads
#     #   thread.join
#     # end
#     threads.each(&:join)
#   end
# }

t2 = Thread.new {
  text_threads = []
  path = Data::PATH.split(" -> ")
  for co in path
    points = co.split(",")
    x = points[0].to_i
    y = points[1].to_i
    
    Square.new(
      x: x * SCALE_F, y: y * SCALE_F,
      size: 1 * SCALE_F,
      color: 'blue',
      opacity: 75,
      z: 2
    )
  
    Thread.new(x, y) { |x, y|
      char = lines[y].chars[x]
      text_threads << Text.new(
        char,
        x: x * SCALE_F, y: y * SCALE_F,
        size: 1 * SCALE_F,
        color: 'white',
        z: 3
      )
    }
  end
  
  # text_threads.each(&:join)
  for thread in text_threads
    thread.join
  end
}

show

# t.join
# t2.join
