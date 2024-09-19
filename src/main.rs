use std::collections::HashSet;

fn has_unique_characters(s: &str) -> bool {
    let mut seen = HashSet::new();

    // Iterates over the characters, converting them to lowercase
    for c in s.chars()
        .map(|c| 
            c.to_lowercase()
                .next()
                .unwrap()) {
        if !seen.insert(c) {
            return false;
        }
    }

    true
}

fn main() {
    let test_cases = vec![
        "abcd",       // 1
        "abCdefAaf",  // 0
        "aabcd",      // 0
    ];

    for test in test_cases {
        let result = has_unique_characters(test);
        println!("{} — {}", test, result);
    }
}
