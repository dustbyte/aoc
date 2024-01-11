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
    let numbers: Vec<u32> = line.chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    numbers[0] * 10 + numbers[numbers.len() - 1]
}

fn main() {
    let sum: u32 = get_content().iter()
        .map(get_number)
        .sum();

    println!("{}", sum);
}
