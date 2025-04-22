use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

/// Clears the terminal screen.
pub fn clear_screen() {
    execute!(io::stdout(), Clear(ClearType::All)).unwrap();
}

/// Reads user input from the terminal and returns it as a u32 value.
/// The input is expected to be an integer choice made by the user.
pub fn read_input() -> u32 {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(0)
}

/// Pauses the program, waiting for the user to press Enter before continuing.
pub fn pause() {
    println!("🔄 Appuyez sur entrée pour continuer...");
    let mut pause_input = String::new();
    io::stdin().read_line(&mut pause_input).unwrap();
}