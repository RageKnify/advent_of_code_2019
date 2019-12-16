use std::io;

fn get_numbers() -> Vec<i32> {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

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

fn interpret(mut vec: Vec<i32>) -> i32 {
    let mut i = 0;
    while i < vec.len() {
        let opcode = vec[i];
        match opcode {
            1 => {
                let idx_a = vec[i + 1] as usize;
                let idx_b = vec[i + 2] as usize;
                let idx_c = vec[i + 3] as usize;
                vec[idx_c] = vec[idx_a] + vec[idx_b];
            }
            2 => {
                let idx_a = vec[i + 1] as usize;
                let idx_b = vec[i + 2] as usize;
                let idx_c = vec[i + 3] as usize;
                vec[idx_c] = vec[idx_a] * vec[idx_b];
            }
            99 => {
                break;
            }
            _ => (),
        }
        i += 4;
    }
    vec[0]
}

fn main() {
    let vec = get_numbers();

    println!("{}", interpret(vec));
}
