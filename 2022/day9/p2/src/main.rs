use std::{fs::File, io::{Result, BufReader, BufRead}, hash::Hash, fmt::Debug};
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
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

    fn follow(&mut self, other: &Self) {
        if self.too_far_from(other) {
            if self.x < other.x {
                self.move_to(&Direction::Right)
            }

            if self.x > other.x {
                self.move_to(&Direction::Left)
            }

            if self.y < other.y {
                self.move_to(&Direction::Up)
            }

            if self.y > other.y {
                self.move_to(&Direction::Down)
            }
        }
    }
}

#[allow(dead_code)]
fn pretty_set<T: Debug>(set: &HashSet<T>) {
    for elem in set {
        println!("{:?}", elem)
    }
}

#[allow(dead_code)]
fn print_map(rope: &Vec<Point>) {
    let mut printed = HashSet::new();

    for j in 0..20 {
        for i in 0..20 {
            let mut found = false;
            for (idx, point) in rope.into_iter().enumerate() {
                let cmp = Point{x: i - 10, y: j - 10};
                if point == &cmp && !printed.contains(&cmp){
                    found =  true;
                    printed.insert(cmp);
                    print!("{}", idx)
                }
            }

            if !found {
                print!{"."}
            }
        }
        print!("\n")
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut visited = HashSet::new();
    let mut prev = Point::default();
    let mut rope = std::iter::repeat_with(|| Point::default()).take(10).collect::<Vec<_>>();

    visited.insert(rope.last().unwrap().clone());

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
                prev.reset(&rope[0]);
                rope[0].move_to(&direction);

                for i in 1..rope.len() {
                    let target = &rope[i-1].clone();

                    rope[i].follow(target)
                }

                visited.insert(rope.last().unwrap().clone());
            }
        }
    }

    println!("{}", visited.len());
    Ok(())
}
