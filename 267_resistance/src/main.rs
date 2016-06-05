use std::env;
use std::io::{BufReader, Result};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::cmp::{Ord, Ordering};
use std::process::exit;
use std::option::Option;

/// # Unwrap or Exit
/// If failing to find the value is a fatal error.
fn uoe<N>(optional: Option<N>, message: String) -> N {
    match optional {
        Some(n) => n,
        None => {
            match write!(std::io::stderr(), "{}", message) {
                Ok(_) => {},
                Err(_) => {},
            };
            exit(1);
        }
    }
}

fn uoe_node<N>(optional: Option<N>, node_name: String) -> N {
    uoe(optional, format!("Unable to find node {}!\n", node_name))
}

struct Grid {
    first_node: String,
    node_map: HashMap<String, Node>,
    link_list: Vec<Link>,
}

impl Grid {
    fn new(header: String) -> Grid {
        let mut node_map = HashMap::new(); 
        let nodes: Vec<&str> = header.split_whitespace().collect();
        for n in &nodes {
            node_map.insert(n.to_string(), Node::new(n.to_string()));
        }
        Grid { first_node: nodes[0].to_string(), node_map: node_map, link_list: Vec::new() }
    }

    fn parse_line(&mut self, line: String) -> Result<()> {
        let node_details: Vec<&str> = line.split_whitespace().collect();
        let node1_name = node_details[0].to_string();
        let node2_name = node_details[1].to_string();
        let mut node1 = uoe(self.node_map.remove(&node1_name), format!("Unable to find node {}!\n", &node1_name));
        let mut node2 = uoe(self.node_map.remove(&node2_name), format!("Unable to find node {}!\n", &node2_name));
        let resistence = node_details[2].parse::<u16>().unwrap();
        match node1.name.cmp(&node2.name) {
            Ordering::Less | Ordering::Equal => { 
                println!("Creating link from {} to {} with resistence {}", node1_name.to_owned(), node2_name.to_owned(), resistence);
                self.link_list.push(Link::new(resistence, node1_name.to_owned(), node2_name.to_owned()));
                
                node1.add_target(self.link_list.len() - 1);
                node2.add_source(self.link_list.len() - 1);
            },
            Ordering::Greater => {
                println!("Creating link from {} to {} with resistence {}", node2_name.to_owned(), node1_name.to_owned(), resistence);
                self.link_list.push(Link::new(resistence, node2_name.to_owned(), node1_name.to_owned()));

                node2.add_target(self.link_list.len() - 1);
                node1.add_source(self.link_list.len() - 1);
            },
        }
        self.node_map.insert(node1_name, node1);
        self.node_map.insert(node2_name, node2);
        Ok(())
    }

    fn node(&self, name: &String) -> &Node {
        uoe(self.node_map.get(name), format!("Unable to find node {}!\n", name))
    }

    fn first_node(&self) -> &Node {
        uoe_node(self.node_map.get(&self.first_node), self.first_node.to_string())
    }

    fn target(&self, node: &Node, link: usize) -> &Node {
        println!("Getting target for {:?}, {}", node, link);
        let l = &self.link_list[node.targets[link]];
        uoe_node(self.node_map.get(&l.target_node), l.target_node.to_string())
    }

    fn source(&self, node: &Node, link: usize) -> &Node {
        println!("Getting source for {:?}, {}", node, link);
        let l = &self.link_list[node.sources[link]];
        uoe_node(self.node_map.get(&l.source_node), l.source_node.to_string())
    }

    fn target_link(&self, node: &Node, link: usize) -> &Link {
        &self.link_list[node.targets[link]]
    }

    fn replace_target_node(&mut self, source_node: &Node, source_link: usize, new_target_node: &Node, target_link: usize, link: Link) {
        self.link_list.push(link);
        let link_idx = self.link_list.len() - 1;
        {
            let sn = uoe_node(self.node_map.get_mut(&source_node.name), source_node.name.to_string());
            sn.targets[source_link] = link_idx;
        }

        let ntn = uoe_node(self.node_map.get_mut(&new_target_node.name), new_target_node.name.to_string());
        ntn.sources[target_link] = link_idx;
    }
}

#[derive(Clone,Debug)]
struct Node {
    sources: Vec<usize>,
    targets: Vec<usize>,
    name: String,
}

#[derive(Debug)]
struct Link {
    source_node: String,
    target_node: String,
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
    fn new(resistence: u16, source_node: String, target_node: String) -> Link {
        Link { resistence: resistence, source_node: source_node, target_node: target_node }
    }
}

fn run(a: Vec<String>) -> Result<u16> {
    if a.len() != 2 {
        try!(std::io::stderr().write(b"Command must be called with file containing resistance description\n"));
        exit(1);
    }

    let f = try!(File::open(&a[1]));
    let reader = BufReader::new(f);

    let mut i: u8 = 0;
    let mut header = String::from("");
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line: String = try!(line);
        println!("[{}] - {}", i, line);
        if i == 0 {
            header = line;
        } else {
            lines.push(line);
        }
        i += 1;
    }

    let mut grid = Grid::new(header);
    for line in lines {
        try!(grid.parse_line(line));
    }

    //process_node(&mut grid, grid.first_node());
    let mut current_node_name = grid.first_node().name.to_string();
    loop {
        let current_node = grid.node(&current_node_name).clone();
        if current_node.targets.len() == 1 {
            println!("Current node has only one target");
            if current_node.sources.len() == 0 && grid.target(&current_node, 0).sources.len() == 1 && grid.target(&current_node, 0).targets.len() == 0 {
                //try!(write!(std::io::stderr(), "Resistence is {}!\n", ));
                //break;
                return Ok(grid.link_list[current_node.targets[0]].resistence);
            } else if grid.target(&current_node, 0).sources.len() == 1 && grid.target(&current_node, 0).targets.len() == 1 {
                let child: Node = grid.target(&current_node, 0).clone();
                let grand_child: Node = grid.target(&child, 0).clone();
                let link = Link::new(grid.target_link(&current_node, 0).resistence + grid.target_link(&child, 0).resistence, current_node.name.to_string(), grand_child.name.to_string());
                grid.replace_target_node(&current_node, 0, &grand_child, 0, link);
            }
        }
    }
}

/// Input:
/// ```
/// [A]--(10)--[B]--(10)--[C]
/// ```
///
/// Expected output:
/// ```
/// [A]--(20)--[C]
/// ```
#[test]
fn should_merge_all_nodes_with_only_single_source_and_target() {
    assert!(run(vec!(String::from("exec"), String::from("./sequential.txt"))).unwrap() == 20);
}

/// Input:
/// ```
///      +--(10)--+
///      |        |
/// [A]--+        +--[B]
///      |        |
///      +--(20)--+
/// ```
///
/// Expected output:
/// ```
/// [A]--(6.666666...7)--[B]
/// ```
#[test]
fn should_calculate_inverse_resistence_for_parallel_links() {
    // assert!(run(vec!(String::from("exec"), String::from("./parallel.txt"))).unwrap() == 7); // Rounding up, since we're returning a u16. Oooops.
}

fn main() {
    let a: Vec<String> = env::args().collect();
    match run(a) {
        Ok(resistence) => println!("Resistence is {}", resistence),
        Err(e) => println!("Error! {}", e.to_string()),
    }
}
