/*
 * Gather simple info:
 * - How many sides
 * - How many rolls
 */
use std::io::stdin;

fn main() {
    println!("Hello, world! Let's roll a dice:");
    let sides = read_number("How many sides should your dice have?", 2, 20);
    let tries = read_number("How many times should we roll the dice?", 1, 255);

    println!("Dice's rollng:");

    for i in 1..=tries {
        println!("{i}: {}", roll(sides));
    }
}

fn roll(sides: u32) -> u32 {
    rand::random_range(1..=sides)
}

fn read_number(prompt: &str, min: u32, max: u32) -> u32 {
    println!("{prompt}");
    loop {
        let mut input = String::new();
        
        let bytes = stdin().read_line(&mut input).unwrap();
        if bytes == 0 {
            std::process::exit(1);
        }

        match input.trim().parse() {
            Ok(n) => {
                if n >= min && n <= max {
                    break n;
                } else {
                    println!("Please pick a number between {min} and {max}.");
                }
            }
            Err(_) => {
                println!("That's not a number! Try again.");
            }
        }
    }
}
