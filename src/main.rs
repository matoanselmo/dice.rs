/*
 * Gather simple info:
 * - How many sides
 * - How many rolls
 */
use clap::Parser;
use owo_colors::OwoColorize;

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

    println!(
        "\n{} Rolling {}d{}...\n",
        "🎲".bold(),
        args.tries.to_string().cyan().bold(),
        args.sides.to_string().cyan().bold()
    );

    let tries = args.tries;

    for i in 1..=tries {
        let result = roll(args.sides);

    let styled_result = if result == args.sides && args.sides > 1 {
            result.to_string().green().bold().to_string()
        } else if result == 1 && args.sides > 1 {
            result.to_string().red().bold().to_string()
        } else {
            result.to_string().cyan().bold().to_string()
        };
        
        println!("  {} Roll #{i}: {}", "•".dimmed(), styled_result);
    }
}

fn roll(sides: u32) -> u32 {
    rand::random_range(1..=sides)
}
