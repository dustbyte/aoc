use std::{fs, process::exit};
use regex::Regex;

const PATTERN: &str = r"mul\((\d{1,3}), ?(\d{1,3})\)";

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
    let content = get_content().join("\n");
    let re = Regex::new(PATTERN).unwrap();
    let result: u32 = re.captures_iter(&content)
        .map(|c| {
            let lhs = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let rhs = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
            lhs * rhs
        }).sum();

    println!("{}", result)
}
