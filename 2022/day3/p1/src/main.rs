use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::collections::HashSet;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String,
}

fn get_priority(c: char) -> u32 {
    match c {
        'a' ..= 'z' => c as u32 - 'a' as u32 + 1,
        'A' ..= 'Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("Invalid character {}", c)
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut sum: u32 = 0;
    let file = File::open(args.filename)?;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            let line_slice = line.strip_suffix("\n").unwrap_or(line.as_str());
            let left: HashSet<char> = line_slice[0..(line.len()/2)].chars().into_iter().collect();
            let right: HashSet<char> = line_slice[(line.len()/2)..].chars().into_iter().collect();

            let common = left.intersection(&right).into_iter().next().unwrap();
            sum += get_priority(*common);
        }
    }
    println!("{}", sum);
    Ok(())
}
