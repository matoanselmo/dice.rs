use std::{collections::BTreeMap, thread, time::Duration};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;

/// Roll some dice and see how they land
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of sides on each die
    #[arg(short, long, default_value_t = 6, value_parser = clap::value_parser!(u32).range(1..))]
    sides: u32,

    /// How many times to roll
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..))]
    tries: u32,

    /// Delay between rolls in milliseconds
    #[arg(short, long, default_value_t = 100)]
    delay: u64,
}

fn main() {
    let args = Args::parse();
    let tries = args.tries;
    let bar = ProgressBar::new_spinner();
    let mut counts: BTreeMap<u32, u32> = BTreeMap::new();

    println!(
        "\n{} Rolling {}d{}...\n{} {}{}\n",
        "🎲".bold(),
        args.tries.cyan().bold(),
        args.sides.cyan().bold(),
        "   Delay".dimmed(),
        args.delay.dimmed().bold(),
        "ms".dimmed().bold()
    );

    bar.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "⠈", "⠉", "⠋", "⠓", "⠒", "⠐", "⠐", "⠒", "⠖", "⠦", "⠤", "⠠", "⠠", "⠤", "⠦", "⠖",
                "⠒", "⠐", "⠐", "⠒", "⠓", "⠋", "⠉", "⠈", "✔",
            ]),
    );

    bar.enable_steady_tick(Duration::from_millis(args.delay));

    for i in 1..=tries {
        let result = roll(args.sides);
        *counts.entry(result).or_insert(0) += 1;

        let styled_result = style_result(result, &args);

        thread::sleep(Duration::from_millis(args.delay));

        bar.set_message(format!("Rolling: {i}x {styled_result}"));
    }

    let mut text = String::from("Done:\n");

    for (result, count) in counts {
        let styled_result = style_result(result, &args);
        text.push_str(&format!("  Rolled {styled_result} {count}x\n"));
    }

    bar.finish_with_message(text.trim_end().to_string());
}

fn roll(sides: u32) -> u32 {
    rand::random_range(1..=sides)
}

fn style_result(result: u32, args: &Args) -> String {
    if result == args.sides && args.sides > 1 {
        result.to_string().green().bold().to_string()
    } else if result == 1 && args.sides > 1 {
        result.to_string().red().bold().to_string()
    } else {
        result.to_string().cyan().bold().to_string()
    }
}
