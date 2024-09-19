fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let input = "главрыба";
    println!("Original: {}", input);
    println!("Reversed: {}", reverse_string(input));
}
