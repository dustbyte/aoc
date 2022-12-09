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

fn scenic_score(field: &TreeField, x: usize, y: usize) -> u32 {
    if x == 0 || x == field.len() - 1 || y == 0 || y == field[0].len() - 1 {
        return 0
    }

    let val = field[x][y];
    let mut score = 1;

    let mut top = 0;
    for i in (0..x).rev() {
        let tree_val = field[i][y];
        top += 1;
        if tree_val >= val {
            break
        }
    }
    score *= top;

    let mut bottom = 0;
    for i in x+1..field.len() {
        let tree_val = field[i][y];
        bottom += 1;
        if tree_val >= val {
            break
        }
    }
    score *= bottom;

    let mut left = 0;
    for j in (0..y).rev() {
        let tree_val = field[x][j];
        left += 1;
        if tree_val >= val {
            break
        }
    }
    score *= left;

    let mut right = 0;
    for j in y+1..field[0].len() {
        let tree_val = field[x][j];
        right += 1;
        if tree_val >= val {
            break
        }
    }
    score *= right;

    score
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

    let mut max = 0;

    for x in 0..field.len() {
        for y in 0..field[0].len() {
            let score = scenic_score(&field, x, y);
            if score > max {
                max = score
            }
        }
    }

    println!("{}", max);
    Ok(())
}
