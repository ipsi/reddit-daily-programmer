use std::env;
use std::io::{self, BufReader, Result};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::cmp::{Ord, Ordering};

struct Node<'a> {
    source: Vec<&'a Node<'a>>,
    target: Vec<&'a Node<'a>>,
    resistence: u16,
    name: String,
}

impl<'a> Node<'a> {
    fn new(name: String, resistence: u16) -> Node<'a> {
        Node { name: name, resistence: resistence, source: Vec::new(), target: Vec::new() }
    }

    fn add_source(&mut self, node: &'a Node<'a>) {
        self.source.push(node);
    }

    fn add_target(&mut self, node: &'a Node<'a>) {
        self.target.push(node);
    }

    fn set_resistance(&mut self, resistence: u16) {
        self.resistence = resistence;
    }
}

fn run() -> Result<()> {
    let mut start: String = String::from("~");
    let a: Vec<String> = env::args().collect();
    if a.len() != 2 {
        try!(std::io::stderr().write(b"Command must be called with file containing resistance description\n"));
        std::process::exit(1);
    }

    let f = try!(File::open(&a[1]));
    let reader = BufReader::new(f);

    let mut i: u8 = 0;
    let mut node_map = HashMap::new();
    for line in reader.lines() {
        let line: String = try!(line);
        println!("[{}] - {}", i, line);
        if i == 0 {
           let nodes = line.split_whitespace(); 
           for node_name in nodes {
              start = match start.cmp(&node_name.to_string()) {
                  Ordering::Less => start,
                  Ordering::Equal => start,
                  Ordering::Greater => node_name.to_string(),
              };
              node_map.insert(node_name.to_string(), Node::new(node_name.to_string(), 1));
           }
        } else {
            let node_details: Vec<&str> = line.split_whitespace().collect();
            let node = node_map.get(&node_details[0].to_string());
            match node {
                Some(n) => {},
                None => {},
            };
        }
        i += 1;
    }
    println!("Will start with node [{}]", start);
    for (key, value) in node_map {
        println!("{}", key);
    }
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => std::process::exit(0),
        Err(e) => println!("Error! {}", e.to_string()),
    }
}
