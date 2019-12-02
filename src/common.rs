use std::io;

/// Collects from stdin into a String until an empty line is encountered
pub fn read_stdin_to_string() -> String {
    let mut input = String::new();
    let mut line = String::new();

    println!("Enter puzzle input followed by an empty line:");
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
