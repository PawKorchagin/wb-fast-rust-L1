fn main() {
    let sorted_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let target = 5;
    match sorted_array.binary_search(&target) {
        Ok(index) => println!("Element {} found at index {}", target, index),
        Err(_) => println!("Element {} not found", target),
    }

    let target_not_found = 11;
    match sorted_array.binary_search(&target_not_found) {
        Ok(index) => println!("Element {} found at index {}", target_not_found, index),
        Err(_) => println!("Element {} not found", target_not_found),
    }
}
