# dice.rs

[![CI](https://github.com/matoanselmo/dice.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/matoanselmo/dice.rs/actions/workflows/ci.yml) [![rust-reportcard](https://rust-reportcard.xuri.me/badge/github.com/matoanselmo/dice.rs)](https://rust-reportcard.xuri.me/report/github.com/matoanselmo/dice.rs)

> This is an application I created to learn and practice rust!

A small command-line dice roller written in Rust. Roll any die as many times as you like, watch a spinner while it rolls, then get a tally of the results.

## Installation

Requires a Rust toolchain that supports edition 2024 (Rust 1.85+).

```sh
cargo install --path .
```

Or just run it from the repo:

```sh
cargo run -- [OPTIONS]
```

## Usage

```sh
dice [OPTIONS]
```

| Option | Short | Default | Description |
|---|---|---|---|
| `--sides <N>` | `-s` | `6` | Number of sides on each die (must be at least 1) |
| `--tries <N>` | `-t` | `1` | How many times to roll (must be at least 1) |
| `--delay <MS>` | `-d` | `100` | Delay between rolls, in milliseconds |
| `--help` | `-h` | | Show help |
| `--version` | `-V` | | Show version |

### Examples

Roll a single six-sided die:

```sh
dice
```

Roll a d20 ten times:

```sh
dice -s 20 -t 10
```

Roll a d6 a thousand times with no delay:

```sh
dice -t 1000 -d 0
```

## Output

While rolling, a spinner shows the current roll. When it finishes, you get how many times each number came up, sorted from lowest to highest:

```
🎲 Rolling 10d20...
   Delay 100ms

✔ Done:
  Rolled 1 1x
  Rolled 7 2x
  Rolled 20 1x
  ...
```

## Development

Format the code (this rewrites files in place):

```sh
cargo fmt
```

Check formatting without changing anything (this is what CI runs):

```sh
cargo fmt --check
```

Run the linter, treating warnings as errors like CI does:

```sh
cargo clippy -- -D warnings
```

CI runs these checks plus `cargo build` and `cargo test` on every push and pull request to `main`.

## License

MIT, see [LICENSE](LICENSE).
