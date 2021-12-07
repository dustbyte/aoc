#!/usr/bin/env elixir

defmodule Vents do
  # same column, going up
  def unroll({{x, y}, {x, z}}) when y < z do
    for i <- y..z do {x, i} end
  end

  # same column, going down
  def unroll({{x, y}, {x, z}}) when y > z do
    for i <- z..y do {x, i} end
  end

  # same line, going right
  def unroll({{x, y}, {z, y}}) when x < z do
    for i <- x..z do {i, y} end
  end

  # same line, going left
  def unroll({{x, y}, {z, y}}) when x > z do
    for i <- z..x do {i, y} end
  end

  # diag up right
  def unroll({{x1, y1}, {x2, y2}}) when x2 > x1 and y2 > y1 do
    for i <- 0..(x2-x1) do {x1 + i, y1 + i} end
  end

  # diag up left
  def unroll({{x1, y1}, {x2, y2}}) when x2 < x1 and y2 > y1 do
    for i <- 0..(x1-x2) do {x1 - i, y1 + i} end
  end

  # diag down left
  def unroll({{x1, y1}, {x2, y2}}) when x2 < x1 and y2 < y1 do
    for i <- 0..(x1-x2) do {x1 - i, y1 - i} end
  end

  # diag down right
  def unroll({{x1, y1}, {x2, y2}}) when x2 > x1 and y2 < y1 do
    for i <- 0..(x2-x1) do {x1 + i, y1 - i} end
  end

  def unroll(_) do
    nil
  end
end

if length(System.argv) < 1 do
  Kernel.exit "usage main.exs filename"
end

lines = File.read!(hd System.argv)
        |> String.split("\n")
        |> Enum.filter(fn x -> x != "" end)
        |> Enum.map(fn line ->
          [_, x1, y1, x2, y2] = Regex.run(~r/(\d+),(\d+) -> (\d+),(\d+)/, line)
            {
              {
                elem(Integer.parse(x1), 0),
                elem(Integer.parse(y1), 0)
              },
              {
                elem(Integer.parse(x2), 0),
                elem(Integer.parse(y2), 0)
              }
            }
        end
        )

points = Enum.reduce(lines, %{}, fn(line, map) ->
  points = Vents.unroll(line)
  if points != nil do
    Enum.reduce(points, map, fn (point, map) ->
      Map.update(map, point, 1, &(&1+1))
    end)
  else
    map
  end
end)

IO.puts Enum.reduce(points, 0, fn({_, value}, count) ->
  if value >= 2 do
    count + 1
  else
    count
  end
end)
