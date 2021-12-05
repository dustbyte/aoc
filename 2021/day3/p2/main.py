#!/usr/bin/env python

"""
Ugly but works
"""

from argparse import ArgumentParser

def filter_numbers(numbers, predicate):
    pos = 0

    while True:
        one_list=[]
        zero_list=[]
        if len(numbers) == 1:
            return numbers[0]

        for number in numbers:
            if number[pos] == '0':
                zero_list.append(number)
            else:
                one_list.append(number)

        numbers = predicate(zero_list, one_list, pos)
        pos += 1

def least_predicate(left_list, right_list, pos):
    if len(left_list) < len(right_list):
        return left_list
    elif len(right_list) < len(left_list):
        return right_list

    if left_list[0][pos] == '0':
        return left_list
    return right_list

def most_predicate(left_list, right_list, pos):
    if len(left_list) > len(right_list):
        return left_list
    elif len(right_list) > len(left_list):
        return right_list

    if left_list[0][pos] == '1':
        return left_list
    return right_list


def main():
    parser = ArgumentParser()
    parser.add_argument('input_file')
    args = parser.parse_args()

    with open(args.input_file) as handle:
        numbers = [line.strip() for line in handle]
        oxygen = filter_numbers(numbers, most_predicate)
        carbon_dioxyde = filter_numbers(numbers, least_predicate)

        print(int(oxygen, base=2) * int(carbon_dioxyde, base=2))


if __name__ == "__main__":
    main()
