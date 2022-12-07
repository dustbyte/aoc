use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use std::collections::HashSet;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            let mut buffer = Vec::<char>::new();

            for (idx, c) in line.chars().enumerate() {
                buffer.insert(0, c.clone());

                if buffer.len() >= 4 {
                    let set = HashSet::<char>::from_iter(buffer.iter().cloned());

                    if set.len() == 4 {
                        println!("{}", idx + 1);
                        break;
                    }
                    buffer.pop();
                }
            }
        }
    }

    Ok(())
}
