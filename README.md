# Rust Guessing Game

![cover](cover.png)

## Overview

This is a simple command-line guessing game written in Rust. The objective is to guess a randomly generated number between 1 and 100. The game demonstrates basic Rust concepts such as variable bindings, user input handling, loops, and conditional logic.

## Getting Started

### Prerequisites

- Ensure you have Rust installed on your computer. If not, you can install it from [here](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository: `git clone https://your-repository-url/guessing_game.git`
2. Navigate to the project directory:`cd guessing_game`

### Running the Game

Execute the following command in the project directory:

```bash
cargo run
```

This will compile and run the game. Follow the on-screen instructions to play.

## How to Play

- After running the game, you'll be prompted to guess a number.
- Enter your guess and press `Enter`.
- The game will inform you if your guess is too high, too low, or correct.
- If your guess is not correct, you'll be prompted to guess again.
- The game continues until you guess the correct number.

## Project Structure

- `src/main.rs`: Contains the main logic of the guessing game.
- `Cargo.toml`: Configuration file for Rust projects, including dependencies.

## Dependencies

- `rand`: A Rust crate used to generate random numbers.

## Learn More

To learn more about Rust, visit the [official Rust documentation](https://doc.rust-lang.org/).