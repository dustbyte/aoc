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
    let mut linebuf = Vec::<HashSet<char>>::new();

    let file = File::open(args.filename)?;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            if linebuf.len() < 3 {
                let chars: HashSet<char> = line.chars().into_iter().collect();
                linebuf.push(chars);
            }

            if linebuf.len() == 3 {
                let common = linebuf
                    .iter()
                    .skip(1)
                    .fold(linebuf[0].clone(), |acc, hs| {
                        acc.intersection(hs).cloned().collect()
                    });
                sum += get_priority(common.into_iter().next().unwrap());
                linebuf.clear();
            }
        }
    }
    println!("{}", sum);
    Ok(())
}
