#!/usr/bin/env python
# -*- coding: utf-8 -*-

from argparse import ArgumentParser
from collections import defaultdict


def build_paths(links, paths, current, node, twice_allowed):
    if node == 'end':
        paths.append(current + [node])
        return
    if node.islower() and node in current:
        if not twice_allowed or node == 'start':
            return
        twice_allowed = False
    for link in links[node]:
        build_paths(links, paths, current + [node], link, twice_allowed)


def main():
    parser = ArgumentParser()
    parser.add_argument('input_file')
    args = parser.parse_args()

    with open(args.input_file) as handle:
        links = defaultdict(list)

        for line in handle:
            line = line.strip()
            left, right = line.split('-')
            links[left].append(right)
            links[right].append(left)

        paths = []
        build_paths(links, paths, [], 'start', False)
        print(len(paths))

        paths = []
        build_paths(links, paths, [], 'start', True)
        print(len(paths))


if __name__ == "__main__":
    main()
