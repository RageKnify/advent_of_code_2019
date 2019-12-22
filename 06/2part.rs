use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Orbital {
    dad: String,
    children: Vec<String>,
}

impl Orbital {
    fn add_child<'a>(&'a mut self, child_name: &'a String) {
        self.children.push(String::from(child_name));
    }
}

fn parse_input(inp_file: &str) -> HashMap<String, Orbital> {
    let file = match File::open(&inp_file) {
        Err(_) => panic!("Couldn't open {}", inp_file),
        Ok(f) => f,
    };
    let reader = BufReader::new(file);

    let mut inp = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut it = line.split(")");
        let dad = match it.next() {
            None => break,
            Some(st) => st,
        };
        let child = match it.next() {
            None => break,
            Some(st) => st,
        };
        let dad_orb = match inp.get_mut(dad) {
            Some(orb) => orb,
            None => {
                let dad_orb = Orbital {
                    dad: String::from("No father"),
                    children: Vec::new(),
                };
                inp.insert(String::from(dad), dad_orb);
                match inp.get_mut(dad) {
                    None => {
                        eprintln!("Just now added dad is missing.");
                        std::process::exit(1);
                    }
                    Some(orb) => orb,
                }
            }
        };
        dad_orb.add_child(&String::from(child));
        let _child_orb = match inp.get_mut(child) {
            Some(orb) => {
                orb.dad = String::from(dad);
                orb
            }
            None => {
                let child_orb = Orbital {
                    dad: String::from(dad),
                    children: Vec::new(),
                };
                inp.insert(String::from(child), child_orb);
                match inp.get_mut(child) {
                    None => {
                        eprintln!("Just now added child is missing.");
                        std::process::exit(1);
                    }
                    Some(orb) => orb,
                }
            }
        };
    }
    return inp;
}

fn calc_distance(map: HashMap<String, Orbital>) -> Option<i32> {
    let curr = match map.get("YOU") {
        Some(orb) => orb,
        None => {
            eprintln!("Can't find YOU");
            std::process::exit(1);
        }
    };
    let mut dists: HashMap<String, i32> = HashMap::new();
    dists.insert(String::from("YOU"), 0);
    let mut stack = curr.children.to_vec();
    stack.push(curr.dad.clone());
    for name in &stack {
        dists.insert(name.clone(), 1);
    }
    while !stack.is_empty() {
        let curr_name = stack.remove(0);
        if curr_name == "No father" {
            continue;
        }
        let curr = match map.get(&curr_name) {
            Some(orb) => orb,
            None => {
                eprintln!("Can't find current orbital");
                std::process::exit(1);
            }
        };
        let h = match dists.get(&curr_name) {
            None => {
                eprintln!("Can't find {}'s height", curr_name);
                std::process::exit(1);
            }
            Some(h) => *h,
        };
        for name in &curr.children {
            if !dists.contains_key(name) {
                dists.insert(String::from(name), h + 1);
                stack.push(name.clone());
            }
        }
        if !dists.contains_key(&curr.dad) {
            dists.insert(curr.dad.clone(), h + 1);
            stack.push(curr.dad.clone());
        }
        if dists.contains_key("SAN") {
            let ret = match dists.get("SAN") {
                Some(i) => Some(*i),
                None => None,
            };
            return ret;
        }
    }
    return None;
}

fn main() {
    let mut it = std::env::args();
    it.next();
    let input_file = match it.next() {
        None => {
            eprintln!("Missing file parameter.");
            eprintln!("Usage: ./1part $INPUT_FILE");
            std::process::exit(1);
        }
        Some(input_file) => input_file,
    };
    let input = parse_input(input_file.as_str());
    let transfers = match calc_distance(input) {
        Some(i) => i - 2,
        None => -1,
    };
    println!("{}", transfers);
}
