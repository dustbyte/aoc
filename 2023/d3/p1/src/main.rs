use std::{fs, process::exit, default, cmp, mem::Discriminant, iter::Map, collections::{HashMap, HashSet}};

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

fn get_symbols(content: &Vec<String>) -> HashMap<(i32, i32), HashSet<u32>> {
    let mut symbols = HashMap::new();

    for (row, line) in content.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if !c.is_digit(10) && c != '.' {
                symbols.insert((row as i32, col as i32), HashSet::new());
            }
        }
        symbols.insert((row as i32, line.len() as i32), HashSet::new());
    }

    symbols
}

fn register_part(parts: &mut HashMap<u32, Vec<(i32, i32)>>, buf: &mut Vec<u32>, row: i32, col: i32) {
    if buf.len() > 0 {
        let number = buf.iter().fold(0, |acc, val| acc * 10 + val);

        if !parts.contains_key(&number) {
            parts.insert(number, vec![]);
        }

        for i in 1..(buf.len() + 1) {
            parts.get_mut(&number).unwrap().push((row as i32, col as i32 - i as i32));
        }

        buf.clear()
    }
}

fn get_parts(content: &Vec<String>) -> HashMap<u32, Vec<(i32, i32)>> {
    let mut parts = HashMap::new();

    for (row, line) in content.iter().enumerate() {
        let mut buf = Vec::<u32>::new();

        for (col, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                buf.push(c.to_digit(10).unwrap())
            } else {
                register_part(&mut parts, &mut buf, row as i32, col as i32)
            }
        }

        register_part(&mut parts, &mut buf, row as i32, line.len() as i32)
    }

    parts
}

fn main() {
    let content = get_content();
    let mut symbols = get_symbols(&content);
    let parts = get_parts(&content);

    for (number, coords) in parts.iter() {
        for coord in coords.iter() {
            for pair in [(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)] {
                let key = (coord.0 + pair.0, coord.1 + pair.1);
                if symbols.contains_key(&key) {
                    symbols.get_mut(&key).unwrap().insert(*number);
                }
            }
        }
    }

    let mut sorted_symbols = symbols.keys().collect::<Vec<&(i32, i32)>>();
    sorted_symbols.sort();


    //for (value, coords) in parts.iter() {
        //if [830, 976].contains(value) {
            //println!("{} = {:?}", &value, &coords)
        //}
    //}

    //for (num, coords) in parts.iter() {
        //println!("{}: {:?}", num, coords)
    //}

    //for sym in sorted_symbols {
        //println!("{:?} = {:?}", sym, symbols.get(sym).unwrap());
    //}

    let sum: u32 = symbols.values().flatten().sum();

    println!("{}", sum);
}
