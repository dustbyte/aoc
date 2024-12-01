use std::{cmp::Ordering, collections::HashMap, default, fmt::{write, Display}, fs, iter::zip, process::exit};

fn read_file(path: String) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_file_name() -> String {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: {} input_file", &args[0]);
        exit(-1)
    }

    args[1].clone()
}

fn get_content() -> Vec<String> {
    read_file(get_file_name())
}

#[derive(Debug, Default)]
enum Direction {
    #[default]
    None,
    Start,
    N,
    S,
    E,
    W,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

type Pipe = [Direction; 2];
type Map = Vec<Vec<Pipe>>;

fn print_map(map: &Map) {
    for line in map.iter() {
        for pipe in line.iter() {
            print!("[{:<5} {:<5}]", pipe[0].to_string(), pipe[1].to_string());
        }
        println!("");
    }
}

fn get_initial_pipe(map: &Map, start: [usize; 2]) -> Pipe {
    let mut pipe = [Direction::None, Direction::None];
    let mut cur = 0;

    if start[0] > 0 {
        match map[start[0] - 1][start[1]] {
            [_, Direction::S] | [Direction::S, _] => {
                pipe[cur] = Direction::N;
                cur += 1;
            },
            _ => ()
        }
    }

    if start[0] + 1 < map.len() {
        match map[start[0] + 1][start[1]] {
            [_, Direction::N] | [Direction::N, _] => {
                pipe[cur] = Direction::S;
                cur += 1;
            },
            _ => ()
        }
    }

    if start[1] > 0 {
        match map[start[0]][start[1] - 1] {
            [_, Direction::E] | [Direction::E, _] => {
                pipe[cur] = Direction::W;
                cur += 1;
            },
            _ => ()
        }
    }

    if start[1] + 1 < map[0].len() {
        match map[start[0]][start[1] + 1] {
            [_, Direction::W] | [Direction::W, _] => {
                pipe[cur] = Direction::E;
            },
            _ => ()
        }
    }

    pipe
}

fn main() {
    let mut pos = [0, 0];

    let mut map: Map = get_content().iter().enumerate().map(|(y, line)|
        line.chars().enumerate().map(|(x, c)|
            match c {
                'S' => {
                    pos = [y, x];
                    [Direction::Start, Direction::Start]
                },
                '|' => [Direction::N, Direction::S],
                '-' => [Direction::W, Direction::E],
                'L' => [Direction::N, Direction::E],
                'J' => [Direction::N, Direction::W],
                '7' => [Direction::S, Direction::W],
                'F' => [Direction::S, Direction::E],
                _ => [Direction::None, Direction::None],
            }
        ).collect::<Vec<Pipe>>()
    ).collect();

    let pipe = get_initial_pipe(&map, pos);

    println!("{:?}", pos);
    print_map(&map);
    println!("{:?}", pipe);
}
