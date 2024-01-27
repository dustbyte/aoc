use std::{cmp::Ordering, collections::HashMap, default, fs, iter::zip, process::exit};

fn read_file(path: String) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_file_name() -> String {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: {} input_file", &args[0]);
        exit(-1)
    }

    args[1].clone()
}

fn get_content() -> Vec<String> {
    read_file(get_file_name())
}

#[derive(Default, Debug)]
struct Tree {
    mapping: HashMap<String, u32>,
    items: Vec<[u32; 2]>,
}

fn build_tree(map: &[String]) -> Tree {
    let mut tree = Tree {
        mapping: HashMap::new(),
        items: vec![],
    };
    let mut current = 0u32;

    for line in map.iter() {
        let item = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        for element in [item, left, right] {
            if let None = tree.mapping.get(element) {
                tree.mapping.insert(element.into(), current);
                tree.items.push([0, 0]);
                current += 1;
            };
        }

        let item_nr = tree.mapping.get(item).unwrap();
        let left_nr = tree.mapping.get(left).unwrap();
        let right_nr = tree.mapping.get(right).unwrap();

        tree.items[(*item_nr) as usize] = [*left_nr, *right_nr];
    }

    tree
}

fn get_instructions(line: &str) -> Vec<u32> {
    line.chars().map(|c| match c {
        'L' => 0,
        _ => 1
    }).collect()
}

fn main() {
    let content = get_content();
    let instructions = get_instructions(&content[0]);
    let tree = build_tree(&content[2..]);

    let mut current = tree.mapping["AAA"];
    let goal = tree.mapping["ZZZ"];
    let mut pc = 0;

    while current != goal {
        let item = tree.items[current as usize];
        current = item[instructions[pc % instructions.len()] as usize];
        pc += 1;
    }

    println!("{}", pc);
}
