#!/usr/bin/env python
# -*- coding: utf-8 -*-

from argparse import ArgumentParser
from heapq import heappush, heappop, heapify


def path_cost(cave):
    visited = set()
    visitees = []
    xmax = len(cave[0]) - 1
    ymax = len(cave) - 1
    start_risk = cave[0][0]

    heappush(visitees, (start_risk, (0, 0)))

    while visitees:
        risk, node = heappop(visitees)

        if node == (xmax, ymax):
            return risk - start_risk

        if node in visited:
            continue

        visited.add(node)

        x, y = node

        for xnext, ynext in ((x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)):
            if (0 <= xnext <= xmax) and (0 <= ynext <= ymax):
                next_risk = risk + cave[ynext][xnext]
                node = (next_risk, (xnext, ynext))
                heappush(visitees, node)


def unfold_map(cave):
    new_cave = cave.copy()
    cave_width = len(cave[0])
    cave_length = len(cave)

    for y in range(cave_length):
        for x in range(cave_width * 4):
            new_cave[y].append((new_cave[y][x % cave_width] + (x // cave_width)) % 9 + 1)

    for y in range(cave_length * 4):
        new_cave.append([(x + (y // cave_length)) % 9 + 1 for x in new_cave[y % cave_length]])

    return new_cave


def main():
    parser = ArgumentParser()
    parser.add_argument('input_file')
    args = parser.parse_args()

    with open(args.input_file) as handle:
        cave = []
        for line in handle:
            cave.append([int(c) for c in line.strip()])

        print(path_cost(cave))

        new_cave = unfold_map(cave)
        print(path_cost(new_cave))


if __name__ == "__main__":
    main()
