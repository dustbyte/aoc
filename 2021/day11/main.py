#!/usr/bin/env python
# -*- coding: utf-8 -*-

from argparse import ArgumentParser

def explore(grid, flashed, x, y):
    flashes = 0
    if x < 0 or y < 0 or x >= len(grid[0]) or y >= len(grid):
        return flashes

    position = (y, x)
    if position not in flashed:
        grid[y][x] += 1
        if grid[y][x] > 9:
            flashes += 1
            grid[y][x] = 0
            flashed.add(position)
            flashes += explore(grid, flashed, x, y - 1)
            flashes += explore(grid, flashed, x + 1, y - 1)
            flashes += explore(grid, flashed, x + 1, y)
            flashes += explore(grid, flashed, x + 1, y + 1)
            flashes += explore(grid, flashed, x, y + 1)
            flashes += explore(grid, flashed, x - 1, y + 1)
            flashes += explore(grid, flashed, x - 1, y)
            flashes += explore(grid, flashed, x - 1, y - 1)

    return flashes

def step(grid):
    flashed = set()
    flashes = 0
    for y in range(len(grid)):
        for x in range(len(grid[y])):
            flashes += explore(grid, flashed, x, y)

    return flashes

def print_grid(grid):
    for line in grid:
        for cell in line:
            print(cell, end="")
        print()
    print()

def check_grid(grid):
    acc = 0
    for line in grid:
        for cell in line:
            acc += cell

    return acc

def main():
    parser = ArgumentParser()
    parser.add_argument('input_file')
    args = parser.parse_args()

    with open(args.input_file) as handle:
        grid = []
        for line in handle:
            line = line.strip()
            grid.append([])
            for number in line:
                grid[-1].append(int(number))

        flashes = 0
        step_count = 0
        while True:
            flashes += step(grid)
            if step_count == 99:
                print(flashes)
            if check_grid(grid) == 0:
                print(step_count + 1)
                return
            step_count += 1



if __name__ == "__main__":
    main()
