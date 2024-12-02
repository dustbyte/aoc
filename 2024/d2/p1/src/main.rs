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

fn same_sign(lhs: i32, rhs: i32) -> bool {
    (lhs >= 0 && rhs >= 0) || (lhs < 0 && rhs < 0)
}

fn main() {
    let reports = get_content()
        .iter()
        .map(|report| {
            report
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        }).collect::<Vec<Vec<i32>>>();

    let count = reports.iter().map(|report|{
        let first_delta = report[0] - report[1];
        let abs_delta = first_delta.abs();
        if abs_delta < 1 || abs_delta > 3 {
            return 0;
        }

        for idx in 1..(report.len() - 1) {
            let delta = report[idx] - report[idx + 1];
            let abs_delta = delta.abs();

            if !same_sign(first_delta, delta) || abs_delta < 1 || abs_delta > 3 {
                return 0;
            }
        }
        1
    }).sum::<usize>();

    println!("{:#?}", count)
}
