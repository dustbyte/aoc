#!/usr/bin/env ruby

require 'pp'

if ARGV.length < 1
  abort("usage: #{$0} file")
end

contents=File.read(ARGV[0]).squeeze(' ').split("\n\n")

numbers = contents[0].split(',').map(&:to_i)
boards = contents[1..].map do |board|
  sum = 0
  board = board.squeeze(" ").split("\n").map do |row|
    row.strip.split(" ").map do |cell|
      value = cell.to_i
      sum += value
      value
    end
  end
  {
    sum: sum,
    board: board,
    rows: [5, 5, 5, 5, 5],
    cols: [5, 5, 5, 5, 5],
    score: 0,
    number: 0
  }
end

def score_board(board, numbers)
  numbers.each_with_index do |number, idx|
    5.times.each do |x|
      5.times.each do |y|
        if board[:board][y][x] == number
          board[:sum] -= number
          board[:rows][y] -= 1
          board[:cols][x] -= 1

          if board[:rows].include?(0) || board[:cols].include?(0)
            return {score: board[:sum] * number, index: idx}
          end
        end
      end
    end
  end
end

scores = boards.map do |board|
  score_board(board, numbers)
end

puts "First to win: " + scores.sort_by {|score| score[:index]}.first[:score].to_s
puts "Last to win: " + scores.sort_by {|score| -score[:index]}.first[:score].to_s
