use std::{fs, process::exit};

fn get_content() -> Vec<String> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("usage: {} input_file", &args[0]);
        exit(-1)
    }

    fs::read_to_string(&args[1])
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let content = get_content();
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in content {
        let mut parts = line.split_whitespace();

        left.push(parts.next().unwrap().parse::<u32>().unwrap());
        right.push(parts.next().unwrap().parse::<u32>().unwrap());
    }

    let result: u32 = left.iter().map(|num| {
        right.iter().map(|elem| if elem == num { num } else { &0 }).sum::<u32>()
    }).sum();

    println!("{}", result);
}
