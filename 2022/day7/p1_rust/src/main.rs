use std::borrow::Borrow;
use std::io::{BufReader, Result, BufRead};
use std::cell::RefCell;
use std::rc::{Rc,Weak};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String
}

trait Node {
    fn name(&self) -> &String;
    fn size(&self) -> usize;
}

struct Directory {
    name: String,
    parent: RefCell<Weak<Directory>>,
    children: Vec<Box<dyn Node>>
}

impl Directory {
    fn new(name: String, parent: Option<Rc<Directory>>) -> Self {
        let parent: RefCell<Weak<Directory>> = match parent {
            None => RefCell::new(Weak::new()),
            Some(parent) => {
                RefCell::new(Rc::downgrade(&parent))
            }
        };

        Self {
            name,
            parent,
            children: Vec::new()
        }
    }

    fn path(&self) -> String {
        let mut vec_path: Vec<String> = vec![];

        println!("{:?}", self.parent.borrow().upgrade().unwrap().name());

        "".into()
    }
}

impl Node for Directory {
    fn size(&self) -> usize {
        let mut size = 0;

        for child in self.children.iter() {
            size += child.size()
        }

        size
    }

    fn name(&self) -> &String {
        &self.name
    }
}

struct File {
    name: String,
    size: usize,
    parent: Weak<Directory>
}

impl Node for File {
    fn size(&self) -> usize {
        self.size
    }

    fn name(&self) -> &String {
        &self.name
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    //let file = std::fs::File::open(args.filename)?;
    //for line in BufReader::new(file).lines() {
        //if let Ok(line) = line {
            //let tokens: Vec<&str> = line.split(' ').collect();

            //match tokens[0] {
                //"$" => {
                    //match tokens[1] {
                        //"cd" => {
                            //if tokens[2] != "/" {
                            //}
                        //}
                        //_ => ()
                    //}
                //}
                //_ => panic!("Unknown token {}", tokens[0])
            //}
        //}
    //}

    let root = Rc::new(Directory::new("/".into(), None));
    let a = Rc::new(Directory::new("a".into(), Some(root)));

    // upgrade returns None
    println!("{:?}", (*a.parent.borrow()).upgrade().unwrap().name());

    Ok(())
}
