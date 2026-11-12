/*
 * Gather simple info:
 * - How many sides
 * - How many rolls 
 */

use rand;

fn main() {
    println!("Hello, world!");
    println!("dice: {}", roll(6))
}

fn roll(sides: u32) -> u32 {
    rand::random_range(1..=sides)
}
