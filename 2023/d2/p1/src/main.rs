use std::{fs, process::exit, default, cmp};

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

#[derive(Default, Debug)]
struct GameData {
    id: u32,
    max_red: u32,
    max_blue: u32,
    max_green: u32
}

fn parse_line(line: &str) -> GameData {
    let tokens = line.split(" ").collect::<Vec<&str>>();

    let mut game = GameData{
        id: tokens[1][..tokens[1].len() - 1].parse::<u32>().unwrap(),
        max_red: Default::default(),
        max_green: Default::default(),
        max_blue: Default::default(),
    };

    for idx in 2..tokens.len() {
        if let Some(number) = tokens[idx].parse::<u32>().ok() {
            let color = tokens[idx + 1].replace(",", "").replace(";", "");

            match color.as_str() {
                "red" => game.max_red = cmp::max(game.max_red, number),
                "green" => game.max_green = cmp::max(game.max_green, number),
                "blue" => game.max_blue = cmp::max(game.max_blue, number),
                _ => panic!("Unknown color {}", color),
            }
        }
    }

    //println!("{:?}", game);
    game
}

fn main() {
    let limits = GameData {
        id: 0,
        max_red: 12,
        max_green: 13,
        max_blue: 14,
    };

    let sum: u32 = get_content().iter()
        .map(|line| parse_line(line))
        .filter(|line| line.max_red <= limits.max_red && line.max_green <= limits.max_green && line.max_blue <= limits.max_blue)
        .map(|line| line.id)
        .sum();

    println!("{}", sum)
}
