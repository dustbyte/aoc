use std::{io::{Result, BufReader, BufRead}, fs::File};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String
}

#[derive(Debug)]
enum Operation {
    Wait,
    Add(i32)
}

struct CPU {
    register: i32,
    cycle: i32,
    ops: Vec<Operation>
}

impl CPU {
    fn new() -> Self {
        Self {
            register: 1,
            cycle: 1,
            ops: vec![]
        }
    }

    fn signal_strength(&self) -> i32 {
        self.cycle * self.register
    }

    fn push_op(&mut self, op: Operation) {
        self.ops.insert(0, op)
    }

    fn next_op(&mut self) -> Option<Operation> {
        self.ops.pop()
    }

    fn has_ops(&self) -> bool {
        self.ops.len() > 0
    }

    fn tick(&mut self) -> i32 {
        match self.next_op() {
            Some(op) => match op {
                Operation::Wait => (),
                Operation::Add(num) => {
                    self.register += num;
                }
            },
            None => panic!("Operation queue is empty")
        }
        self.cycle += 1;
        self.cycle
    }

    fn add(&mut self, rhs: i32) {
        self.push_op(Operation::Wait);
        self.push_op(Operation::Add(rhs));
    }

    fn noop(&mut self) {
        self.push_op(Operation::Wait)
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut pos = 0;
    let mut cpu = CPU::new();
    let file = File::open(args.filename)?;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            let tokens: Vec<&str> = line.split(" ").collect();

            match tokens[..] {
                [_] => cpu.noop(),
                [_, num] => cpu.add(num.parse::<i32>().unwrap()),
                _ => panic!("Uknown operation {}", tokens[0])
            }

            while cpu.has_ops() {
                let hpos = pos % 40;

                if hpos == 0 && pos != 0{
                    print!("\n");
                }

                if hpos >= cpu.register - 1 && hpos <= cpu.register + 1 {
                    print!("#");
                } else {
                    print!(".")
                }

                cpu.tick();
                pos += 1;
            }
        }
    }
    print!("\n");
    Ok(())
}
