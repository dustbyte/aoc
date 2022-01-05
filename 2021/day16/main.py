#!/usr/bin/env python
# -*- coding: utf-8 -*-

from argparse import ArgumentParser


def to_bits(string):
    content = []

    for c in string:
        content.append(int(c, base=16) & 0xF)

    return ''.join(["{:04b}".format(byte) for byte in content])


class StreamReader:

    def __init__(self, stream):
        self.stream = stream
        self.cur = 0

    def can_read(self, size):
        return self.cur + size < len(self.stream)

    def read_bits(self, size, peak=False):
        value = self.stream[self.cur:self.cur+size]
        if not peak:
            self.cur += size
        return value

    def read_int(self, size, peak=False):
        return int(self.read_bits(size, peak), base=2)


def read_number(reader):
    number = 0
    done = False

    while not done:
        bits = reader.read_bits(5)
        if bits[0] == '0':
            done = True
        number = (number << 4) + (int(bits[1:], base=2) & 0xF)

    return number


def read_operator(reader):
    version = 0
    operands = []

    if reader.read_int(1) == 0:
        length = reader.read_int(15)
        target = reader.cur + length
        while reader.cur < target:
            ver, op = read_packet(reader)
            version += ver
            operands.append(op)
    else:
        count = reader.read_int(11)
        for _ in range(count):
            ver, op = read_packet(reader)
            version += ver
            operands.append(op)

    return version, operands


def read_packet(reader):
    version = reader.read_int(3)
    packet_type = reader.read_int(3)

    if packet_type == 4:
        return version, read_number(reader)

    ver, operands = read_operator(reader)

    value = 0
    match packet_type:
        case 0:
            value = sum(operands)
        case 1:
            value = 1
            for op in operands:
                value *= op
        case 2:
            value = min(operands)
        case 3:
            value = max(operands)
        case 5:
            value = 1 if operands[0] > operands[1] else 0
        case 6:
            value = 1 if operands[0] < operands[1] else 0
        case 7:
            value = 1 if operands[0] == operands[1] else 0

    return version + ver, value


def main():
    parser = ArgumentParser()
    parser.add_argument('input_file')
    args = parser.parse_args()

    with open(args.input_file) as handle:
        content = to_bits(next(handle).strip())
        version_sum, value = read_packet(StreamReader(content))

        print(version_sum, value)


if __name__ == "__main__":
    main()
