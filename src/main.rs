use num_bigint::BigInt;

fn main() {
    // Define large numbers a and b
    let a = BigInt::from(2).pow(30);  // 2^30
    let b = BigInt::from(2).pow(25);  // 2^25
    
    // Addition
    let add_result = &a + &b;
    println!("Addition: {} + {} = {}", a, b, add_result);
    
    // Subtraction
    let sub_result = &a - &b;
    println!("Subtraction: {} - {} = {}", a, b, sub_result);
    
    // Multiplication
    let mul_result = &a * &b;
    println!("Multiplication: {} * {} = {}", a, b, mul_result);
    
    // Division
    let div_result = &a / &b;
    println!("Division: {} / {} = {}", a, b, div_result);
}
