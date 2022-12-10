use std::{fs::File, io::{Result, BufReader, BufRead}, hash::Hash};
use std::collections::HashSet;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    filname: String
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn move_to(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1
        }
    }

    fn reset(&mut self, other: &Self) {
        self.x = other.x;
        self.y = other.y
    }

    fn too_far_from(&self, other: &Self) -> bool {
        (self.x - other.x).abs() > 1 || (self.y - other.y).abs() > 1
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut head = Point{x: 0, y: 0};
    let mut prev_head = Point{x: 0, y: 0};
    let mut tail = Point{x: 0, y: 0};
    let mut visited = HashSet::new();

    visited.insert(tail.clone());

    let file = File::open(args.filname)?;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            let elems: Vec<&str> = line.split(' ').collect();

            let direction = match elems[0] {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("Unknown direction {}", elems[0])
            };

            for _ in 0..elems[1].parse::<i32>().unwrap() {
                prev_head.reset(&head);
                head.move_to(&direction);
                if tail.too_far_from(&head) {
                    tail.reset(&prev_head);
                    visited.insert(tail.clone());
                }
            }
        }
    }

    println!("{}", visited.len());
    Ok(())
}
