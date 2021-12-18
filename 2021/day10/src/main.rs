extern crate clap;

use std::{fs::File, io::BufRead};
use std::io::BufReader;
use clap::{App,Arg};

fn parse(line: &[u8]) -> (u64, Vec<char>) {
    let mut stack = Vec::<char>::new();

    for c in line.into_iter() {
        match *c as char {
            '(' | '{' | '[' | '<' => stack.push(*c as char),
            ')' => if stack.pop() != Some('(') { return (3, stack) },
            ']' => if stack.pop() != Some('[') { return (57, stack) },
            '}' => if stack.pop() != Some('{') { return (1197, stack) },
            '>' => if stack.pop() != Some('<') { return (25137, stack) },
            _ => ()
        }
    }

    (0, stack)
}

fn completion_score(stack: &mut Vec<char>) -> u64 {
    let mut score = 0;

    while let Some(c) = stack.pop() {
        score = score * 5 + match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0
        }
    }
    score
}

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("d10")
        .arg(Arg::with_name("input_file")
            .takes_value(true)
            .required(true)
        ).get_matches();

    let file = File::open(matches.value_of("input_file").unwrap())?;
    let lines = BufReader::new(file).lines();
    let mut score: u64 = 0;
    let mut completions = Vec::<u64>::new();

    for line in lines {
        let (result, mut stack) = parse(line.unwrap().as_bytes());

        score += result;

        if result == 0 {
            let comp_score = completion_score(&mut stack);
            completions.push(comp_score);
        }
    }

    println!("{}", score);
    completions.sort();
    println!("{}", completions[completions.len() / 2]);

    Ok(())
}
