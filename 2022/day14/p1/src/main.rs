use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut solids = HashSet::new();
    let mut lowest = 0;

    let file = File::open(args.filename)?;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            let mut prev: Option<_> = Option::None;

            for pos in line.split(" -> ") {
                let [x, y]: [u32; 2] = pos.split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                if y > lowest {
                    lowest = y;
                }

                if let Some((px, py)) = prev {
                    if px == x {
                        let (start, stop) = if py < y { (py, y) } else { (y, py) };
                        for j in start+1..stop {
                            solids.insert((x, j));
                        }
                    }

                    if py == y {
                        let (start, stop) = if px < x { (px, x) } else { (x, px) };

                        for i in start+1..stop {
                            solids.insert((i, y));
                        }
                    }
                }

                solids.insert((x, y));
                prev = Option::Some((x, y))
            }
        }
    }

    let mut dropped = false;
    let mut rested_units: u32 = 0;

    while !dropped {
        let (mut x, mut y) = (500, 0);
        let mut rested = false;

        while !rested {
            if y >= lowest {
                println!("{}", rested_units);
                dropped = true;
                break;
            }

            if !solids.contains(&(x, y + 1)) {
                y += 1;
                continue;
            }

            if !solids.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
                continue;
            }

            if !solids.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
                continue;
            }

            rested_units += 1;
            solids.insert((x, y));
            rested = true;
        }
    }
    Ok(())
}
