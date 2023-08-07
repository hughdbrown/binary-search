use std::io;
use std::io::Write;

// Prompt the user for an usize.
pub fn get_i32(prompt: &str) -> usize {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<usize>()
        .expect("Error parsing integer");
}


