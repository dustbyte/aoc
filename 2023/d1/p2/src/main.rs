use core::num;
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

fn get_number(line: &String) -> u32 {
    let numbers = digitize(line);

    numbers[0] * 10 + numbers[numbers.len() - 1]
}

fn digitize(line: &str) -> Vec<u32> {
    let mut numbers = vec![];

    for (i, c) in line.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            numbers.push(digit)
        } else {
            for (digit, value) in [("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)] {
                if line[i..].starts_with(digit) {
                    numbers.push(value)
                }
            }
        }
    }

    numbers
}

fn main() {
    let sum: u32 = get_content().iter()
        .map(get_number)
        .sum();

    println!("{}", sum);
}
