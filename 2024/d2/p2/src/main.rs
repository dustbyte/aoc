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

fn in_range(delta: i32) -> bool {
    let abs_delta = delta.abs();
    abs_delta >= 1 && abs_delta <= 3
}

fn is_safe(first_delta: i32, delta: i32) -> bool {
    same_sign(delta, first_delta) && in_range(delta)
}

fn report_is_safe(report: &[i32]) -> bool {
    let first_delta = report[0] - report[1];
    if !in_range(first_delta) {
        return false;
    }

    for idx in 1..(report.len() - 1) {
        let delta = report[idx] - report[idx + 1];

        if !is_safe(first_delta, delta) {
            return false;
        }
    }
    true
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
        if report_is_safe(report) {
            1
        } else {
            // Ugly but works
            for idx in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(idx);
                if report_is_safe(&new_report) {
                    return 1;
                }
            }
            0
        }
    }).sum::<usize>();

    println!("{:#?}", count)
}
