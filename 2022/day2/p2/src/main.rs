use std::fs::File;
use std::io::{prelude::*, BufReader};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String,
}

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

const RULES: [[u32; 3]; 3]= [
    [3, 4, 8],
    [1, 5, 9],
    [2, 6, 7],
];

fn run_match(them: Hand, me: Hand) -> u32 {
    RULES[them as usize][me as usize]
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut sum = 0;
    let file = File::open(args.filename)?;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            let values = line.strip_suffix("\n").unwrap_or(line.as_str())
            .split(" ")
            .map(|x| {
                match x {
                    "A" | "X" => Hand::Rock,
                    "B" | "Y" => Hand::Paper,
                    "C" | "Z" => Hand::Scissors,
                    _ => panic!("Unexpected value {}", x)
                }
            })
            .collect::<Vec<Hand>>() ;
            sum += run_match(values[0], values[1])
        }
    }
    println!("{}", sum);
    Ok(())
}
