use std::io;

fn main() {
    let mut sum: i32 = 0;

    loop {
        let mut mass = String::new();

        io::stdin()
            .read_line(&mut mass)
            .expect("Failed to read line");

        let mass: i32 = match mass.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        let mut t_fuel: i32 = 0;
        let mut fuel: i32 = (mass / 3) - 2;

        while fuel > 0 {
            t_fuel += fuel;
            fuel = (fuel / 3) - 2;
        }

        sum += t_fuel;
    }

    println!("Result is {}", sum);
}
