use std::{fs, process::exit, collections::HashSet};

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

fn score(card: &str) -> u32 {
    let num_sets = card
        .split_once(":")
        .unwrap()
        .1.split("|")
        .map(|side| side
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap()).collect::<HashSet<u32>>()).collect::<Vec<_>>();

    num_sets[0].intersection(&num_sets[1]).count() as u32
}

fn main() {
    let content = get_content();
    let mut scratches = vec![1; content.len()];

    for i in 0..content.len() {
        for j in 0..score(&content[i]) {
            scratches[i + j as usize + 1] += scratches[i];
        }
    }

    println!("{}", scratches.iter().sum::<u32>())

}
