use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn get_numbers(inp_file: String) -> Vec<i32> {
    let path = Path::new(&inp_file);
    let display = path.display();
    let mut file = match File::open(path) {
        Err(_) => panic!("Couldn't open {}", inp_file),
        Ok(f) => f,
    };

    let mut line = String::new();
    match file.read_to_string(&mut line) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => (),
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

fn op_3(vec: &mut Vec<i32>, i: usize) -> usize {
    let idx = vec[i + 1] as usize;
    let mut line = String::new();

    print!("Opcode 3, write to vec[{}]: ", idx);
    io::stdout().flush();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let num: i32 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse int"),
    };

    vec[idx] = num;
    i + 2
}

fn op_4(vec: &mut Vec<i32>, i: usize, c: i32) -> usize {
    let val: i32;
    let idx = vec[i + 1];
    if c == 1 {
        val = idx;
    } else {
        val = vec[idx as usize];
    }
    println!("Opcode 4: {}", val);
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

fn interpret(mut vec: Vec<i32>) -> i32 {
    let mut i = 0;
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
                i = op_3(&mut vec, i);
            }
            4 => {
                i = op_4(&mut vec, i, c);
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
                break;
            }
            _ => (),
        }
    }
    vec[0]
}

fn main() {
    let mut it = std::env::args();
    it.next();
    let f = match it.next() {
        None => panic!("Missing file parameter"),
        Some(f) => f,
    };
    let vec = get_numbers(f);
    interpret(vec);
}
