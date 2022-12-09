use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String,
}

type TreeField = Vec<Vec<u32>>;

fn is_visible(field: &TreeField, x: usize, y: usize) -> bool {
    if x == 0 || x == field.len() - 1 || y == 0 || y == field[0].len() - 1 {
        return true
    }

    let val = field[x][y];

    let mut top = true;
    for i in 0..x {
        if field[i][y] >= val {
            top = false;
            break
        }
    }
    if top {
        return true
    }

    let mut bottom = true;
    for i in x+1..field.len() {
        if field[i][y] >= val {
            bottom = false;
            break
        }
    }
    if bottom {
        return true
    }

    let mut left = true;
    for j in 0..y {
        if field[x][j] >= val {
            left = false;
            break
        }
    }
    if left {
        return true
    }

    let mut right = true;
    for j in y+1..field[0].len() {
        if field[x][j] >= val {
            right = false;
            break
        }
    }
    if right {
        return true
    }

    false
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut field = Vec::<Vec<u32>>::new();

    let file = File::open(args.filename)?;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            let mut row = Vec::<u32>::new();

            for c in line.chars() {
                row.push(c.to_digit(10).unwrap())
            }

            field.push(row)
        }
    }

    let mut sum = 0;

    for x in 0..field.len() {
        for y in 0..field[0].len() {
            if is_visible(&field, x, y) {
                sum += 1
            }
        }
    }

    println!("{}", sum);
    Ok(())
}
