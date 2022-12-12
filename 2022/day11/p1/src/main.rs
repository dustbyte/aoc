use std::{io::{Result, BufReader, BufRead}, fs::File};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String
}

#[derive(Default, Debug)]
struct MonkeyTest {
    divider: u32,
    monkey_true: usize,
    monkey_false: usize,
}

#[derive(Debug)]
struct Monkey {
    #[allow(unused)]
    id: usize,
    items: Vec<u32>,
    operation: Vec<String>,
    inspections: u32,
    test: MonkeyTest,
}

impl Monkey {
    fn new(id: usize) -> Self {
        Self {
            id,
            items: vec![],
            operation: vec![],
            inspections: 0,
            test: Default::default(),
        }
    }

    fn hash_items(&self) -> bool {
        self.items.len() > 0
    }

    fn inspect_and_dispatch(&mut self) -> (usize, u32) {
        let item = self.items.remove(0);
        let new = self.run_op(item) / 3;

         let next_monkey = if new % self.test.divider == 0 {
             self.test.monkey_true
         } else {
             self.test.monkey_false
         };

         self.inspections += 1;

         (next_monkey, new)
    }

    fn run_op(&self, old: u32) -> u32 {
        let lval = if self.operation[0] == "old" {
            old
        } else {
            self.operation[0].parse::<u32>().unwrap()
        };

        let rval = if self.operation[2] == "old" {
            old
        } else {
            self.operation[2].parse::<u32>().unwrap()
        };

        match self.operation[1].as_str() {
            "*" => lval * rval,
            "+" => lval + rval,
            op => panic!("Unknown operator {}", op)
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut monkeys: Vec<Monkey> = vec![];
    let file = File::open(args.filename)?;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            let line = line.replace(":", "");
            let line = line.replace(",", "");
            let tokens: Vec<&str> = line.split_whitespace().collect();

            match tokens[..] {
                ["Monkey", num] =>  monkeys.push(Monkey::new(num.parse::<usize>().unwrap())),
                ["Starting", ..] => monkeys.last_mut().unwrap().items = tokens[2..].into_iter().map(|x| (*x).parse::<u32>().unwrap()).collect(),
                ["Operation", ..] => monkeys.last_mut().unwrap().operation = tokens[3..=5].into_iter().map(|x| String::from(*x)).collect(),
                ["Test", _, _, num] => monkeys.last_mut().unwrap().test.divider = num.parse::<u32>().unwrap(),
                ["If", "true", _, _, _, num] => monkeys.last_mut().unwrap().test.monkey_true = num.parse::<usize>().unwrap(),
                ["If", "false", _, _, _, num] => monkeys.last_mut().unwrap().test.monkey_false = num.parse::<usize>().unwrap(),
                _ => ()
            }
        }
    }

    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            while monkeys[idx].hash_items() {
                let (monkey, result) = monkeys[idx].inspect_and_dispatch();
                monkeys[monkey].items.push(result)
            }
        }
    }

    let mut inspections: Vec<u32> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort_by(|a, b| b.cmp(a));

    println!("{}", inspections[0] * inspections[1]);
    Ok(())
}
