use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_numbers(inp_file: String) -> Vec<i32> {
    let file = match File::open(&inp_file) {
        Err(_) => panic!("Couldn't open {}", inp_file),
        Ok(f) => f,
    };
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let line = lines.next().unwrap();

    let line = match line {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Couldn't trim line, {:?}", e.kind());
            std::process::exit(1);
        }
    };
    let numbers = line.trim().split(',');

    let mut vec = Vec::new();
    for num in numbers {
        let num: i32 = match num.parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        vec.push(num);
    }
    vec
}

fn op_1(vec: &mut Vec<i32>, i: usize, _a: i32, b: i32, c: i32) -> usize {
    let val_a: i32;
    let val_b: i32;

    let idx_a = vec[i + 1];
    if c == 1 {
        val_a = idx_a;
    } else {
        val_a = vec[idx_a as usize];
    }

    let idx_b = vec[i + 2];
    if b == 1 {
        val_b = idx_b;
    } else {
        val_b = vec[idx_b as usize];
    }

    let idx_c = vec[i + 3] as usize;
    vec[idx_c] = val_a + val_b;
    i + 4
}

fn op_2(vec: &mut Vec<i32>, i: usize, _a: i32, b: i32, c: i32) -> usize {
    let val_a: i32;
    let val_b: i32;

    let idx_a = vec[i + 1];
    if c == 1 {
        val_a = idx_a;
    } else {
        val_a = vec[idx_a as usize];
    }

    let idx_b = vec[i + 2];
    if b == 1 {
        val_b = idx_b;
    } else {
        val_b = vec[idx_b as usize];
    }

    let idx_c = vec[i + 3] as usize;
    vec[idx_c] = val_a * val_b;
    i + 4
}

fn op_3(vec: &mut Vec<i32>, i: usize, input: &mut Vec<i32>) -> usize {
    let idx = vec[i + 1] as usize;

    let num = input.remove(0);

    vec[idx] = num;
    i + 2
}

fn op_4(vec: &mut Vec<i32>, i: usize, c: i32, output: &mut Vec<i32>) -> usize {
    let val: i32;
    let idx = vec[i + 1];
    if c == 1 {
        val = idx;
    } else {
        val = vec[idx as usize];
    }
    output.push(val);
    i + 2
}

fn op_5(vec: &mut Vec<i32>, i: usize, _a: i32, b: i32, c: i32) -> usize {
    let val_a: i32;
    let val_b: i32;

    let idx_a = vec[i + 1];
    if c == 1 {
        val_a = idx_a;
    } else {
        val_a = vec[idx_a as usize];
    }

    let idx_b = vec[i + 2];
    if b == 1 {
        val_b = idx_b;
    } else {
        val_b = vec[idx_b as usize];
    }

    if val_a != 0 {
        return val_b as usize;
    } else {
        return i + 3;
    }
}

fn op_6(vec: &mut Vec<i32>, i: usize, _a: i32, b: i32, c: i32) -> usize {
    let val_a: i32;
    let val_b: i32;

    let idx_a = vec[i + 1];
    if c == 1 {
        val_a = idx_a;
    } else {
        val_a = vec[idx_a as usize];
    }

    let idx_b = vec[i + 2];
    if b == 1 {
        val_b = idx_b;
    } else {
        val_b = vec[idx_b as usize];
    }

    if val_a == 0 {
        return val_b as usize;
    } else {
        return i + 3;
    }
}

fn op_7(vec: &mut Vec<i32>, i: usize, _a: i32, b: i32, c: i32) -> usize {
    let val_a: i32;
    let val_b: i32;
    let val_c: i32;
    let idx_a = vec[i + 1];

    if c == 1 {
        val_a = idx_a;
    } else {
        val_a = vec[idx_a as usize];
    }

    let idx_b = vec[i + 2];
    if b == 1 {
        val_b = idx_b;
    } else {
        val_b = vec[idx_b as usize];
    }

    let idx_c = vec[i + 3];
    val_c = idx_c;

    if val_a < val_b {
        vec[val_c as usize] = 1;
    } else {
        vec[val_c as usize] = 0;
    }
    return i + 4;
}

fn op_8(vec: &mut Vec<i32>, i: usize, _a: i32, b: i32, c: i32) -> usize {
    let val_a: i32;
    let val_b: i32;
    let val_c: i32;
    let idx_a = vec[i + 1];

    if c == 1 {
        val_a = idx_a;
    } else {
        val_a = vec[idx_a as usize];
    }

    let idx_b = vec[i + 2];
    if b == 1 {
        val_b = idx_b;
    } else {
        val_b = vec[idx_b as usize];
    }

    let idx_c = vec[i + 3];
    val_c = idx_c;

    if val_a == val_b {
        vec[val_c as usize] = 1;
    } else {
        vec[val_c as usize] = 0;
    }
    return i + 4;
}

fn interpret(mut vec: Vec<i32>, mut input: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut output = Vec::new();
    while i < vec.len() {
        let opcode = vec[i];
        let a = (opcode / 10000) % 10;
        let b = (opcode / 1000) % 10;
        let c = (opcode / 100) % 10;
        let op = opcode % 100;
        match op {
            1 => {
                i = op_1(&mut vec, i, a, b, c);
            }
            2 => {
                i = op_2(&mut vec, i, a, b, c);
            }
            3 => {
                i = op_3(&mut vec, i, &mut input);
            }
            4 => {
                i = op_4(&mut vec, i, c, &mut output);
            }
            5 => {
                i = op_5(&mut vec, i, a, b, c);
            }
            6 => {
                i = op_6(&mut vec, i, a, b, c);
            }
            7 => {
                i = op_7(&mut vec, i, a, b, c);
            }
            8 => {
                i = op_8(&mut vec, i, a, b, c);
            }
            99 => {
                return output;
            }
            _ => (),
        }
    }
    return output;
}

pub fn permutations(size: usize) -> Permutations {
    Permutations {
        idxs: (0..size).collect(),
        swaps: vec![0; size],
        i: 0,
    }
}

pub struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() {
                    return None;
                }
                if self.swaps[self.i] < self.i {
                    break;
                }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
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
        Some(file) => file,
    };
    let vec = get_numbers(input_file);
    let permutations = permutations(5).collect::<Vec<_>>();
    let mut max = 0;
    for perm in permutations {
        let mut out = 0;
        for phase in perm {
            let mut inputs = Vec::<i32>::new();
            inputs.push(phase as i32);
            inputs.push(out as i32);
            out = interpret(vec.to_vec(), inputs)[0];
        }
        max = std::cmp::max(out, max);
    }
    println!("max is {}", max);
}
