use std::{cmp::Ordering, collections::HashMap, default, fs, iter::zip, process::exit};

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

fn predict(numbers: &Vec<i32>) -> i32 {
    let mut zero_count = 0;

    let diffs: Vec<i32> = (1..numbers.len()).map(|idx| {
        let val = numbers[idx as usize] - numbers[idx as usize - 1];
        if val == 0 {
            zero_count += 1;
        }
        val
    }
    ).collect();

    //println!("{:?} :: {}", diffs, zero_count);

    if zero_count == diffs.len() {
        return *numbers.last().unwrap();
    }

    *numbers.last().unwrap() + predict(&diffs)
}

fn main() {
    let val: i32 = get_content().iter().map(|line| {
        let vals = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
        predict(&vals)
    }).sum();

    println!("{}", val);
}
