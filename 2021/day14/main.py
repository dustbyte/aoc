#!/usr/bin/env python
# -*- coding: utf-8 -*-

from argparse import ArgumentParser
from collections import Counter
from functools import lru_cache


def compute(template, rules, steps):
    @lru_cache(maxsize=None)
    def count(pair, step):
        counter = Counter()
        if step == 0:
            return counter
        element = rules[pair]
        counter.update([element])
        counter.update(count(pair[0] + element, step - 1))
        counter.update(count(element + pair[1], step - 1))
        return counter

    counter = Counter(template)
    for idx in range(len(template)):
        if idx + 1 < len(template):
            counter.update(count(template[idx] + template[idx + 1], steps))

    most_common = counter.most_common()
    return most_common[0][1] - most_common[-1][1]


def main():
    parser = ArgumentParser()
    parser.add_argument('input_file')
    args = parser.parse_args()

    with open(args.input_file) as handle:
        template = None
        rules = {}
        for line in handle:
            line = line.strip()
            if not template:
                template = list(line)
            else:
                if not line:
                    continue
                pair, insertion = line.split(' -> ')
                rules[pair] = insertion


        print(compute(template[:], rules, 10))
        print(compute(template[:], rules, 40))


if __name__ == "__main__":
    main()
