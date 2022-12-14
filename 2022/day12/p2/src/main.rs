use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};

use clap::Parser;
use pathfinding::prelude::astar;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Point {
    x: usize,
    y: usize,
    value: char
}

type HeighMap = Vec<Vec<Point>>;

fn get(map: &HeighMap, x: i32, y: i32) -> Option<Point> {
    if x >= (map[0].len() as i32) || x < 0 || y < 0 || y >= (map.len() as i32) {
        None
    } else {
        Some(map[y as usize][x as usize].clone())
    }
}

impl Point {
    fn get_neighbors(&self, map: &HeighMap) -> Vec<(Point, u32)> {
        let mut neighbors = Vec::new();
        let x = self.x as i32;
        let y = self.y as i32;
        let value = self.value;


        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            if let Some(point) = get(&map, x + dx, y + dy) {
                if (point.value as i8 - value as i8) <= 1 {
                    neighbors.push((point, 1))
                }
            }
        }

        neighbors
    }

    fn distance(&self, other: &Self) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y) )as u32
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut starts: Vec<Point> = vec![];
    let mut end: Point = Default::default();
    let mut map: HeighMap = Vec::new();

    let file = File::open(args.filename)?;
    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            let row: Vec<Point> = line.chars()
                .enumerate()
                .map(|(x, mut value)| {
                    if value == 'S' {
                        value = 'a';
                    }

                    if value == 'E' {
                        value = 'z';
                        end = Point{x, y, value}
                    }

                    if value == 'a' {
                        starts.push(Point{x, y, value})
                    }

                    Point{x, y, value}
                })
                .collect();
            map.push(row)
        }
    }

    let mut min = u32::MAX;

    for start in starts {
        match astar(
            &start,
            |p: &Point| p.get_neighbors(&map),
            |p| p.distance(&end),
            |p| *p == end
        ) {
            None => (),
            Some((_, length)) => {
                if length < min {
                    min = length
                }
            }
        }

    }

    println!("{}", min);

    Ok(())
}
