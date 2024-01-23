#![feature(iter_array_chunks)]

use std::{fs, process::exit};

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

type RuleSet<'a> = Vec<(&'a str, Vec<(u64, u64, u64)>)>;

fn build_rules(content: &Vec<String>) -> RuleSet {
    let mut rules = vec![];

    for line in content[2..].iter() {
        if line.len() > 0 {
            match line.as_bytes()[0] as char {
                c if c.is_alphabetic() => {
                    rules.push((line.split_whitespace().collect::<Vec<&str>>()[0], vec![]));
                },
                c if c.is_digit(10) => {
                    // Ugly
                    let nums = line.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                    rules.last_mut().unwrap().1.push((nums[0], nums[1], nums[2]))
                },
                _ => continue
            }
        }
    }

    rules
}

fn get_seeds(line: &str) -> Vec<u64> {

    line.split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .array_chunks()
        .map(|[start, size]| start..(start+size))
        .flatten()
        .collect()
}

fn get_score(seed: u64, rules: &RuleSet) -> u64 {
    let mut score = seed;

    for rule in rules {
        //println!("{:#?}", rule.0);
        for item in rule.1.iter() {
            if (item.1..(item.1+item.2)).contains(&score) {
                score = item.0 + (score - item.1);
                //println!("{}", score);
                break
            }
        }
    }

    //println!("{}", score);
    score
}

fn main() {
    let content = get_content();
    let seeds = get_seeds(&content[0]);
    let rules = build_rules(&content);
    let mut score = u64::MAX;

    for seed in seeds {
        let this_score = get_score(seed, &rules);
        score = score.min(this_score);
    }

    println!("{}", score)
}
