use std::{fs, process::exit};
use regex::{Captures, Regex};

const PATTERN: &str = r"mul\((\d{1,3}), ?(\d{1,3})\)|do\(\)|don't\(\)";

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
    let mut enabled = true;

    let result = re.captures_iter(&content)
        .map(move |c| {
            match c.get(0).unwrap().as_str() {

                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        let lhs = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
                        let rhs = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
                        return lhs * rhs;
                    }
                }
            }
            0
        }).sum::<u32>();

        println!("{}", result)
}
