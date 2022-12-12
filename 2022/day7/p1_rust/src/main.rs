use std::borrow::Borrow;
use std::io::{BufReader, Result, BufRead};
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
    parent: Weak<Directory>,
    directories: Vec<Rc<Directory>>
}

impl Directory {
    fn new(name: String, parent: Option<Weak<Directory>>) -> Self {
        let parent: Weak<Directory> = match parent {
            None => Weak::new(),
            Some(parent) => parent
        };

        Self {
            name,
            parent,
            directories: Vec::new()
        }
    }

    fn path(&self) -> String {
        let mut vec_path: Vec<String> = vec![self.name().clone()];
        let mut current = self.parent.upgrade();

        while let Some(parent) = &current {
            vec_path.insert(0, parent.name().clone());
            current = parent.parent.upgrade();
        }

        format!("/{}", vec_path[1..].join("/"))
    }

}

impl Node for Directory {
    fn size(&self) -> usize {
        let mut size = 0;

        for child in self.directories.iter() {
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

    let file = std::fs::File::open(args.filename)?;
    let mut root = Rc::new(Directory::new("/".into(), None));
    let mut cwd = &mut root;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            let tokens: Vec<&str> = line.split(' ').collect();

            match tokens[..] {
                ["$", "ls"] => (),
                ["$", "cd", dir] => {
                    if dir != "/" {

                    }
                },
                ["dir", dirname] => {
                    let mut new = Rc::new(Directory::new(dirname.into(), Some(Rc::downgrade(&cwd.clone()))));
                    cwd.directories.push((&new).clone())
                }
                _ => panic!("Unknown token {}", tokens[0])
            }
        }
    }

    //let root = Rc::new(Directory::new("/".into(), None));
    //let a = Rc::new(Directory::new("a".into(), Some(Rc::downgrade(&root.clone()))));
    //let b = Rc::new(Directory::new("b".into(), Some(Rc::downgrade(&a.clone()))));

    //println!("{}", b.path());

    Ok(())
}
