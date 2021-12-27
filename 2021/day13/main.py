#!/usr/bin/env python
# -*- coding: utf-8 -*-

from argparse import ArgumentParser


def fold_vert(dots, line):
    new_dots = set()

    for dot in dots:
        diff = dot[1] - line
        if diff > 0:
            dot = (dot[0], dot[1] - 2 * diff)
            if dot[1] < 0:
                continue
        new_dots.add(dot)

    return new_dots


def fold_hor(dots, line):
    new_dots = set()

    for dot in dots:
        diff = dot[0] - line
        if diff > 0:
            dot = (dot[0] - 2 * diff, dot[1])
            if dot[0] < 0:
                continue
        new_dots.add(dot)

    return new_dots


def find_maxes(dots):
    x = 0
    y = 0

    for dot in dots:
        if dot[0] > x:
            x = dot[0]
        if dot[1] > y:
            y = dot[1]

    return x, y


def print_transparent(dots):
    max_x, max_y = find_maxes(dots)

    for y in range(max_y + 1):
        for x in range(max_x + 1):
            if (x, y) in dots:
                print('#', end="")
            else:
                print('.', end="")
        print()


def main():
    parser = ArgumentParser()
    parser.add_argument('input_file')
    args = parser.parse_args()

    with open(args.input_file) as handle:
        dots = set()
        folds = []
        searching_dots = True
        for line in handle:
            line = line.strip()
            if not line:
                searching_dots = False
                continue
            if searching_dots:
                x, y = line.split(',')
                dots.add((int(x), int(y)))
            else:
                coord, pos = line.split(' ')[2].split('=')
                folds.append((coord, int(pos)))

        printed = False
        for fold in folds:
            if fold[0] == 'x':
                dots = fold_hor(dots, fold[1])
            else:
                dots = fold_vert(dots, fold[1])
            if not printed:
                print(len(dots))
                printed = True

        print_transparent(dots)


if __name__ == "__main__":
    main()
