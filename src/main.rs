/*
 * Gather simple info:
 * - How many sides
 * - How many rolls
 */
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 6)]
    sides: u32,
    
    #[arg(short, long, default_value_t = 1)]
    tries: u32,
}

fn main() {
    let args = Args::parse();
    
    println!("Hello, world! Let's roll a dice:");
    let sides = args.sides;
    let tries = args.tries;

    println!("Dice's rollng:");

    for i in 1..=tries {
        println!("{i}: {}", roll(sides));
    }
}

fn roll(sides: u32) -> u32 {
    rand::random_range(1..=sides)
}