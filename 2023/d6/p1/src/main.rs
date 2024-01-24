use std::{iter::zip, process::exit, fs};

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
    let times: Vec<u32> = content[0].split_once(":").unwrap().1.trim().split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
    let distances: Vec<u32> = content[1].split_once(":").unwrap().1.trim().split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();

    let result: usize = zip(times, distances)
        .map(|(time, record)|
            (0..time+1)
            .filter(|hold| hold * (time - hold) > record)
            .count())
        .product();

    println!("{:?}", result)
}
