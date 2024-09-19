fn main() {
    let s = "snow dog sun";

    // Fold is a consuming iterator adaptor which applies a function to each element of the iteration, accumulating the result into a new value.

    println!("{}", s.split_whitespace().fold(String::new(), |acc, word| {
        if acc.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", word, acc)
        }
    }));
}
