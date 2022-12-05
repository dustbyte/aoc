use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::RangeInclusive;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String
}

trait Intersects {
    fn intersects_with(&self, rhs: &Self) -> bool;
}

impl<Idx: PartialOrd> Intersects for RangeInclusive<Idx> {
    fn intersects_with(&self, rhs: &Self) -> bool {
        (rhs.end() >= self.start() && rhs.end() <= self.end()) ||
            (rhs.start() >= self.start() && rhs.start() <= self.end())
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut sum: u32 = 0;
    let file = File::open(args.filename)?;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            let ranges: Vec<RangeInclusive<u32>> = line
                .split(",")
                .map(|x| {
                    let bounds: Vec<u32> = x.split('-').map(|x| x.parse::<u32>().unwrap()).collect();
                    RangeInclusive::new(bounds[0], bounds[1])
                })
            .collect();

            if ranges[0].intersects_with(&ranges[1]) || ranges[1].intersects_with(&ranges[0]) {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
    Ok(())
}
