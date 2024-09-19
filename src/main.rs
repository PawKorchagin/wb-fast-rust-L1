use std::io::{self, BufRead};

fn main() {
    let mut unique_strings = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if !unique_strings.contains(&line) {
            unique_strings.push(line);
        }
    }

    // Note: use ctrl + D to end input
    
    for line in unique_strings {
        println!("{}", line);
    }
}
