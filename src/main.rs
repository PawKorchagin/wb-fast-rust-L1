fn main() {
    let mut v = vec![10, 20, 30, 40, 50];
    
    let removed_element = v.remove(2);

    println!("Removed element: {}", removed_element);
    println!("Updated vector: {:?}", v);
}
