use std::{process::exit, fs};

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

fn main() {
    let content = get_content();
    let time: u64 = content[0].split_once(":").unwrap().1.trim().split_whitespace().collect::<Vec<&str>>().join("").parse::<_>().unwrap();
    let record: u64 = content[1].split_once(":").unwrap().1.trim().split_whitespace().collect::<Vec<&str>>().join("").parse::<_>().unwrap();

    let result: usize = (0..time+1)
        .filter(|hold| hold * (time - hold) > record)
        .count();

    println!("{}", result)
}
