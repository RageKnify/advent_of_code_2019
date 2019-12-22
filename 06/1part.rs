use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Orbital {
    name: String,
    dad: String,
}

fn parse_input(inp_file: String) -> Vec<Orbital> {
    let file = match File::open(&inp_file) {
        Err(_) => panic!("Couldn't open {}", inp_file),
        Ok(f) => f,
    };
    let reader = BufReader::new(file);

    let mut inp = Vec::new();
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
        let ob = Orbital {
            name: child.to_string(),
            dad: dad.to_string(),
        };
        inp.push(ob);
    }
    return inp;
}

fn exists<'a>(name: &String, vec: &'a Vec<Orbital>) -> Option<&'a Orbital> {
    for orb in vec {
        if orb.name.as_str() == name {
            return Some(orb);
        }
    }
    return None;
}

fn calc_heights(inp: &mut Vec<Orbital>) -> i32 {
    let mut sum = 0;
    for orb in inp.iter() {
        let mut name = &orb.dad;
        let mut height = 1;
        loop {
            match exists(&name, &inp) {
                Some(dad_orb) => {
                    height += 1;
                    name = &dad_orb.dad;
                }
                None => {
                    break;
                }
            }
        }
        sum += height;
    }
    sum
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
    let mut input = parse_input(input_file);
    let sum = calc_heights(&mut input);
    println!("Number of orbitals is {}", sum);
}
