#[macro_use] extern crate scan_fmt;

use std::rc::{Rc, Weak};
use std::cell::{RefCell};

#[derive(Clone, Debug)]
struct File {
    size: i32,
}

impl  File  {
    fn new(size: i32) -> File {
        File{
            size,
        }
    }
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Rc<RefCell<Directory>>>,
    parent: Option<Weak<RefCell<Directory>>>,
    size: i32
}

impl  Directory  {
    fn new(name: String, parent: Option<Rc<RefCell<Directory>>>) -> Rc<RefCell<Directory>> {
        let weak_parent: Option<Weak<RefCell<Directory>>> = match parent.clone() {
            Some(weak_parent) => Some(Rc::downgrade(&weak_parent)),
            None => None,
        };

        let dir = Rc::new(RefCell::new(Directory {
            name,
            files: Vec::new(),
            directories: Vec::new(),
            parent: weak_parent,
            size: 0,
        }));

        if let Some(parent_strong) = parent.clone() {
            parent_strong.borrow_mut().add_folder(dir.clone());
        }

        dir
    }

    fn add_size(&mut self, size: i32) {
        self.size += size;
        match self.parent.clone() {
            Some(parent) => parent.upgrade().unwrap().borrow_mut().add_size(size),
            None => {},
        };
    }

    fn add_file(&mut self, file: &File) {
        self.files.push(file.clone());
        self.size += file.size;
        match self.parent.clone() {
            Some(parent) => parent.upgrade().unwrap().borrow_mut().add_size(file.size),
            None => {},
        };
    }

    fn add_folder(&mut self, directory: Rc<RefCell<Directory>>) {
        self.directories.push(directory);
    }

    fn get_sizes(&self, sizes: &mut Vec<i32>, limit: i32) {
        if self.size <= limit && self.size > 0 {
            sizes.push(self.size);
        }
        for directory in &self.directories {
            directory.borrow().get_sizes(sizes, limit);
        }
    }

    fn get_sizes_part_2(&self, sizes: &mut Vec<i32>, lower_limit: i32) {
        if self.size > lower_limit {
            sizes.push(self.size);
        }
        for directory in &self.directories {
            directory.borrow().get_sizes_part_2(sizes, lower_limit);
        }
    }

    fn child(&self, name: String) -> Option<Rc<RefCell<Directory>>> {
        for directory in &self.directories {
            if directory.borrow().name == name {
                return Some(directory.clone());
            }
        }
        None
    }
}

fn main() {
    let file = std::fs::read_to_string("input").unwrap();
    let mut lines = file.lines().peekable();

    let mut current_directory: Option<Weak<RefCell<Directory>>> = None;
    let root: Rc<RefCell<Directory>> = Directory::new("/".to_owned(), None);
    
    while let Some(line) = lines.next() {
        let (command, directory_name) = scan_fmt_some!(line, "$ {} {}", String, String);

        if Some("cd".to_owned()) == command {
            if current_directory.is_none() {
                current_directory = Some(Rc::downgrade(&root));
                continue;
            }

            if Some("..".to_owned()) == directory_name {
                current_directory = current_directory.unwrap().upgrade().unwrap().borrow().parent.clone();
            } else if let Some(directory_name) = directory_name {
                if let Some(directory) = current_directory.clone().unwrap().upgrade().unwrap().borrow().child(directory_name.clone()) {
                    current_directory = Some(Rc::downgrade(&directory));
                }
            }

        } else if Some("ls".to_owned()) == command {
            // I am sorry for bringing this monstrosity upon this world (this goes for this entire program)
            // keep going until either there is no next line or first character is a $, aka the end of output
            while lines.peek().is_some() && lines.peek().unwrap().chars().next().unwrap() != '$' {
                let line = lines.next().unwrap();
                let (metadata, name) = scan_fmt!(line, "{} {}", String, String).unwrap();

                if metadata == "dir" {
                    let _ = Directory::new(name.clone(), current_directory.clone().unwrap().upgrade());
                } else {
                    let file = File::new(metadata.parse().unwrap());
                    current_directory.clone().unwrap().upgrade().unwrap().borrow_mut().add_file(&file);
                }
            }
        } else {
            panic!("Unknown command {:?}", command);
        }
    }

    let mut sizes: Vec<i32> = Vec::new();
    let limit = 100_000;
    root.borrow().get_sizes(&mut sizes, limit);

    println!("{:#?}", root);
    println!("{}", sizes.iter().sum::<i32>());

    let mut sizes: Vec<i32> = Vec::new();
    let limit = 30_000_000 - (70_000_000 - root.borrow().size);
    root.borrow().get_sizes_part_2(&mut sizes, limit);
    println!("to reach {limit} we need to remove {}", sizes.iter().min().unwrap());
}
