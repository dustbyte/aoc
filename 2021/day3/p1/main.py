#!/usr/bin/env python

#!/usr/bin/env python
# -*- coding: utf-8 -*-

from argparse import ArgumentParser


def main():
    parser = ArgumentParser()
    parser.add_argument('input_file')
    args = parser.parse_args()

    with open(args.input_file) as handle:
        sums = None
        length = 0
        # required to invert n-bit length
        mask = 0
        for line in handle:
            line = line.strip()
            if not length:
                length = len(line)
                sums = [0] * length
                mask = int('1' * length, base=2)
            for pos, c in enumerate(line):
                if c == '1':
                    sums[pos] += 1
                else:
                    sums[pos] -= 1
        gama = 0
        for pos, bit in enumerate(sums):
            if bit > 0:
                gama |= 1 << (length - 1 - pos)
        epsilon = ~gama & mask
        print(gama * epsilon)


if __name__ == "__main__":
    main()

