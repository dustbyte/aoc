use core::panic;
use std::fs::File;
use std::i32::{MIN, MAX};
use std::io;
use std::io::{BufReader, BufRead};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String
}

#[derive(Debug, Clone)]
enum Item {
    Value(i32),
    None,
    List(Vec<Item>)
}

// BNF
// packet = list
// list = '[' item* [ ',' item ]* ']'
// item = () | '0'-'9' | list
//

fn parse_item(tokens: &Vec<char>, idx: usize) -> (Item, usize) {
    match tokens[idx] {
        '0'..='9' => {
            let mut num: i32 = 0;
            let mut idx = idx;

            while tokens[idx] >= '0' && tokens[idx] <= '9' {
                num = num * 10 + tokens[idx].to_digit(10).unwrap() as i32;
                idx += 1;
            }

            (Item::Value(num), idx)
        },
        '[' => parse_packet(tokens, idx),
        _ => (Item::None, idx + 1)
    }
}

fn parse_packet(tokens: &Vec<char>, idx: usize) -> (Item, usize) {
    let mut packet = Vec::<Item>::new();

    // assumed to be [
    let mut idx = idx + 1;
    let mut done = false;

    while !done {
        let (item, new_idx) = parse_item(tokens, idx);

        match item {
            Item::None => done = true,
            d => packet.push(d),
        }

        idx = new_idx;

        if idx < tokens.len() && tokens[idx] == ',' {
            idx += 1;
        }
    }

    (Item::List(packet), idx)
}

fn compare(left: &Item, right: &Item) -> i32 {
    match (left, right) {
        (&Item::Value(lv), &Item::Value(rv)) => rv - lv ,
        (Item::List(ll), Item::List(rl)) => {
            let mut idx = 0;
            while idx < ll.len() && idx < rl.len() {
                match compare(&ll[idx], &rl[idx]) {
                    d if d != 0 => return d,
                    _ => ()
                }
                idx += 1;
            }

            rl.len() as i32 - ll.len() as i32
        },
        (Item::List(_), &Item::Value(rv)) => compare(left, &Item::List(vec![Item::Value(rv)])),
        (&Item::Value(lv), Item::List(_)) => compare(&Item::List(vec![Item::Value(lv)]), right),
        (_, _) => panic!("There should be no None item"),
    }
}

fn divider(val: i32) -> Item {
    Item::List(vec![
        Item::List(vec![
            Item::Value(val)
        ])
    ])
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let div2 = divider(2);
    let div6 = divider(6);
    let mut packets = vec![div2.clone(), div6.clone()];

    let file = File::open(args.filename)?;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            if line.len() > 0 {
                let (packet, _) = parse_packet(&line.chars().collect(), 0);
                packets.push(packet);
            }
        }
    }

    packets.sort_by(|a, b| {
        match compare(a, b)  {
            0 => std::cmp::Ordering::Equal,
            MIN..=-1 => std::cmp::Ordering::Greater,
            1..=MAX => std::cmp::Ordering::Less,
        }
    });

    let mut key = 1;

    for (idx, packet) in packets.into_iter().enumerate() {
        if compare(&packet, &div2) == 0 || compare(&packet, &div6) == 0 {
            key *= idx + 1;
        }
    }
    println!("{}", key);
    Ok(())
}