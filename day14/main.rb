#!/usr/bin/env ruby

PairReturnVal = Struct.new(:pairs, :letters_added)
def newPairs(pairs, rules)
    new_pairs = Hash.new
    letters_added = Hash.new
    pairs.each do |pair|
        if !rules[pair[0]].nil?
            count = pairs[pair[0]]
            pairs[pair[0]] = 0
            first_char = pair[0][0]
            second_char = pair[0][1]
            first_pair = first_char + rules[pair[0]]
            second_pair = rules[pair[0]] + second_char
            # nil.to_i #=> 0
            new_pairs[first_pair] = new_pairs[first_pair].to_i + count
            new_pairs[second_pair] = new_pairs[second_pair].to_i + count

            letters_added[rules[pair[0]]] = letters_added[rules[pair[0]]].to_i + count
        else
            new_pairs[pair[0]] = new_pairs[pair[0]].to_i + pairs[pair[0]]
        end
    end
    PairReturnVal.new(new_pairs, letters_added)
end

input = File.read "input.txt"
# Parse input
template = ""
rules = Hash.new
input.lines.each do |line|
    line = line.strip
    if !line.include?("->") && line != ""
        template = line
    elsif line != "" 
        split = line.split(" -> ")
        rules[split[0]] = split[1]
    end
end

# Find all pairs currently in the set
pairs = Hash.new
(0..template.length-2).each do |i|
    pairs[template[i]+template[i+1]] = pairs[template[i]+template[i+1]].to_i + 1
end

# Get the initial count of letters in the template
all_letters = Hash.new
template.chars.each do |char|
    all_letters[char] = all_letters[char].to_i + 1
end
# Update pairs and keep track of the added letters
(1..ARGV[0].to_i).each do |i|
    return_val = newPairs(pairs, rules)
    return_val.letters_added.each do |letter|
        all_letters[letter[0]] = all_letters[letter[0]].to_i + letter[1]
    end
    pairs = return_val.pairs
end

max = all_letters.max { |a,b| a[1] <=> b[1] }
min = all_letters.min { |a,b| a[1] <=> b[1] }

puts max[1] - min[1]
