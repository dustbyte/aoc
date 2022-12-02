//use std::fs::File;
//use std::io::prelude::*;

use std::fs::File;
use std::io::{prelude::*, BufReader};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut elves = Vec::new();

    let file = File::open(args.filename)?;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            if elves.len() == 0 || line.len() == 0 {
                elves.push(0)
            }

            if line.len() > 0 {
                let elf = elves.last_mut().unwrap();
                *elf += line.parse::<u32>().unwrap()
            }
        }
    }

    elves.sort_by(|a, b| b.cmp(a));
    let sum = &elves[0..3].iter().fold(0, |acc, x| acc + x);
    println!("{:}", sum);
    Ok(())
}
