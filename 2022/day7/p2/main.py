#!/usr/bin/env python

from argparse import ArgumentParser

class Node:
    def __init__(self, name, parent=None):
        self.name = name
        self.parent = parent

    @property
    def path(self):
        cwd = self
        dirs = []

        while cwd:
            dirs.insert(0, cwd)
            cwd = cwd.parent

        return dirs

    @property
    def pretty_path(self):
        return '/' + '/'.join(self.path)

    def __str__(self):
        return self.name

    def __repr__(self):
        return str(self)

    def tuple_repr(self):
        return f"({self.__class__.__name__.lower()}, size={self.size})"


class Dir(Node):
    def __init__(self, name, parent=None):
        super().__init__(name, parent)
        self.children = []

    def add_file(self, name, size):
        self.children.append(File(name, size, self))

    def add_dir(self, name):
        self.children.append(Dir(name, self))

    def get_child(self, name):
        for child in self.children:
            if child.name == name:
                return child
        raise ValueError(f"File or directory {name} is not a child of {self}")

    @property
    def size(self):
        return sum([child.size for child in self.children])



class File(Node):
    def __init__(self, name, size, parent):
        super().__init__(name, parent)
        self.size = size


def print_hier(cwd):
    spaces = " " * (len(cwd.path) - 1)
    print(f"{spaces}- {cwd.name} {cwd.tuple_repr()}")
    if hasattr(cwd, 'children'):
        for d in cwd.children:
            print_hier(d)


def find_dirs_above(cwd, limit):
    dirs = []

    if cwd.size >= limit:
        dirs = [cwd]

    for child in cwd.children:
        if isinstance(child, Dir):
            dirs.extend(find_dirs_above(child, limit))

    return dirs


def main():
    parser = ArgumentParser()
    parser.add_argument('filename')

    args = parser.parse_args()
    cwd = Dir('/')
    root = cwd

    with open(args.filename) as file:
        for line in file:
            line = line.strip().split(' ')

            if line[0] == "$":
                if line[1] == "cd":
                    if line[2] == "..":
                        if not cwd.parent:
                            raise ValueError(f"Directory {cwd} doesn't have a parent")
                        cwd = cwd.parent
                    elif line[2] != "/":
                        cwd = cwd.get_child(line[2])
            elif line[0] == 'dir':
                cwd.add_dir(line[1])
            elif line[0].isnumeric():
                cwd.add_file(line[1], int(line[0]))

        unused_space = 70000000 - root.size
        target = 30000000 - unused_space
        print(sorted(find_dirs_above(root, target), key=lambda x: x.size)[0].size)

if __name__ == '__main__':
    main()
