use std::{collections::HashMap, fs, process::exit};

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
    mapping: HashMap<String, usize>,
    items: Vec<(String, [usize; 2])>,
}

fn build_tree(map: &[String]) -> Tree {
    let mut tree = Tree {
        mapping: HashMap::new(),
        items: vec![],
    };
    let mut current = 0;

    for line in map.iter() {
        let item = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        for element in [item, left, right] {
            if let None = tree.mapping.get(element) {
                tree.mapping.insert(element.into(), current);
                tree.items.push((element.into(), [0, 0]));
                current += 1;
            };
        }

        let item_nr = tree.mapping.get(item).unwrap();
        let left_nr = tree.mapping.get(left).unwrap();
        let right_nr = tree.mapping.get(right).unwrap();

        tree.items[(*item_nr) as usize].1 = [*left_nr, *right_nr];
    }

    tree
}

fn get_instructions(line: &str) -> Vec<usize> {
    line.chars().map(|c| match c {
        'L' => 0,
        _ => 1
    }).collect()
}

fn gcd(a: usize, b: usize) -> usize {
    let rem = a % b;
    if rem == 0 { b } else { gcd(b, rem) }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn print_nodes(tree: &Tree, nodes: &Vec<usize>) {
    println!("[{}]", nodes.iter().map(|n| tree.items[*n].0.clone()).collect::<Vec<String>>().join(", "));
}

fn run_node(tree: &Tree, instructions: &Vec<usize>, node: usize) -> usize {
    let mut count = 0;
    let mut current = node;

    loop {
        for next in instructions.iter() {
            count += 1;
            current = tree.items[current].1[*next];
            if tree.items[current].0.ends_with("Z") {
                return count;
            }
        }
    }
}

fn main() {
    let content = get_content();
    let instructions = get_instructions(&content[0]);
    let tree = build_tree(&content[2..]);
    let nodes = tree.mapping.iter()
        .filter(|(n, _)| n.chars().nth(2).unwrap() == 'A')
        .map(|(_, p)| *p).collect::<Vec<usize>>();
    let counts: Vec<usize> = nodes.iter().map(|n| run_node(&tree, &instructions, *n)).collect();

    println!("{}", counts.iter().fold(1, |acc, n| lcm(acc, *n)))
}
