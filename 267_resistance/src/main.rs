use std::env;
use std::io::{self, BufReader, Result};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::cmp::{Ord, Ordering};
use std::process::exit;

struct Node {
    sources: Vec<usize>,
    targets: Vec<usize>,
    name: String,
}

struct Link {
    sourceNode: String,
    targetNode: String,
    resistence: u16,
}

impl Node {
    fn new(name: String) -> Node {
        Node { name: name, sources: Vec::new(), targets: Vec::new() }
    }

    fn add_source(&mut self, link: usize) {
        self.sources.push(link);
    }

    fn add_target(&mut self, link: usize) {
        self.targets.push(link);
    }
}

impl Link {
    fn new(resistence: u16, sourceNode: String, targetNode: String) -> Link {
        Link { resistence: resistence, sourceNode: sourceNode, targetNode: targetNode }
    }

    fn set_resistance(&mut self, resistence: u16) {
        self.resistence = resistence;
    }
}

fn create_nodes(line: &String) -> HashMap<String, Node> {
   let mut node_map = HashMap::new(); 
            let nodes = line.split_whitespace(); 
            for n in nodes {
                let node_name = n.to_string();
                node_map.insert(node_name.to_string(), Node::new(node_name.to_string()));
                // start = match start.cmp(&node_name) {
                //    Ordering::Less | Ordering::Equal => start,
                //    Ordering::Greater => node_name,
                // };
            }
            return node_map;
}

fn run() -> Result<()> {
    // let mut start = String::from("~");
    let a: Vec<String> = env::args().collect();
    if a.len() != 2 {
        try!(std::io::stderr().write(b"Command must be called with file containing resistance description\n"));
        exit(1);
    }

    let f = try!(File::open(&a[1]));
    let reader = BufReader::new(f);

    let mut i: u8 = 0;
    let mut node_map = HashMap::new();
    let mut link_list = Vec::new();
    for line in reader.lines() {
        let line: String = try!(line);
        println!("[{}] - {}", i, line);
        if i == 0 {
            node_map = create_nodes(&line);
        } else {
            let node_details: Vec<&str> = line.split_whitespace().collect();
            let node1_name = node_details[0].to_string();
            let node2_name = node_details[1].to_string();
            let mut node1: Node;
            let mut node2: Node;
            match node_map.remove(&node1_name) {
                Some(n) => node1 = n,
                None => {
                    try!(write!(std::io::stderr(), "Unable to find node {}!\n", &node1_name));
                    exit(1);
                },
            };
            match node_map.remove(&node2_name) {
                Some(n) => node2 = n,
                None => {
                    try!(write!(std::io::stderr(), "Unable to find node {}!\n", &node2_name));
                    exit(1);
                },
            };
            let resistence = node_details[2].parse::<u16>().unwrap();
            match node1.name.cmp(&node2.name) {
                Ordering::Less | Ordering::Equal => { 
                    let l = Link::new(resistence, node1_name.to_owned(), node2_name.to_owned());
                    link_list.push(l);
                    
                    node1.add_target(link_list.len());
                    node2.add_source(link_list.len());
                },
                Ordering::Greater => {
                    let l = Link::new(resistence, node2_name.to_owned(), node1_name.to_owned());
                    link_list.push(l);

                    node2.add_target(link_list.len());
                    node1.add_source(link_list.len());
                },
            }
            node_map.insert(node1_name, node1);
            node_map.insert(node2_name, node2);
        }
        i += 1;
    }
    // println!("Will start with node [{}]", start);
    for (key, value) in node_map {
        println!("{}", key);
    }
    Ok(())
}

fn process_node(node: &mut Node) {
    if node.targets.len() == 1 {
        if node.sources.len() == 1 {
            return;
        } // else if node.targets[0].targetNode.sources.len() == 1 && node.targets[0].targetNode.targets.len() == 1 {
            // let link = Link::new(node.targets[0].resistence + node.targets[0].targetNode.targets[0].resistence, node, node.targets[0].targetNode.targets[0].targetNode);
            // node.targets[0].targetNode.targets[0].targetNode.sources[0] = &link;
            // node.targets[0] = link;
        // }
    }
}

fn main() {
    match run() {
        Ok(_) => exit(0),
        Err(e) => println!("Error! {}", e.to_string()),
    }
}
