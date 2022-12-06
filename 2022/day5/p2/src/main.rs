use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String,
}

fn build_state(lines: &mut Lines<BufReader<File>>) -> Vec<Vec<char>> {
    let mut crates = Vec::<Vec<char>>::new();
    let mut width = 0;

    'up: for line in lines {
        if let Ok(line) = line {
            // We have reached the end of the description
            if line.len() == 0 {
                break
            }

            if width == 0 {
                // we add the final virtual whitespace
                width = (line.len() + 1) / 4;
                for _ in 0..width {
                    crates.push(Vec::<char>::new());
                }
            }

            for (idx, c) in line.chars().enumerate() {
                // We ignore the label line
                if c.is_digit(10) {
                    continue 'up;
                }

                if c.is_alphabetic() {
                    // We want the top crate at the end to then use push/pop
                    crates[idx / 4].insert(0, c);
                }
            }
        }
    }
    crates
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let mut lines = BufReader::new(file).lines();

    let mut crates = build_state(&mut lines);

    for line in lines {
        if let Ok(line) = line {
            let fields: Vec<&str> = line .split(" ").collect();
            let qty = fields[1].parse::<usize>().unwrap();
            let src = fields[3].parse::<usize>().unwrap() - 1;
            let dst = fields[5].parse::<usize>().unwrap() - 1;

            for pos in (1..=qty).rev() {
                let len = crates[src].len();
                let val = crates[src].remove(len - pos);
                crates[dst].push(val)
            }
        }
    }
    for c in crates {
        print!("{}", c[c.len() - 1]);
    }
    println!("");
    Ok(())
}
