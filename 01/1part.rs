use std::io;

fn main() {
    let mut sum: u32 = 0;

    loop {
        let mut mass = String::new();

        io::stdin()
            .read_line(&mut mass)
            .expect("Failed to read line");

        let mass: u32 = match mass.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        let fuel: u32 = (mass / 3) - 2;
        sum += fuel;
    }

    println!("Result is {}", sum);
}
