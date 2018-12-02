//! Common code used between the various puzzles

use std::io;

/// Reads from stdin until an empty line is detected
///
/// Returns the read data as a string
pub fn read_stdin_to_string() -> String {
    let mut input = String::new();
    let mut line = String::new();

    println!("enter puzzle input followed by an empty line:");
    loop {
        io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "" {
            break;
        }
        input.push_str(&line);
        line.clear();
    }

    input
}
