fn main() {
    let mut n = 0u64;

    for i in 0..65 {
        match turn_on_i_th_bit(&mut n, i) {
            Ok(_) => println!("Turn on {}-th bit: {}", i, n),
            Err(error) => println!("Error while turning on bit with index {}, error: {}", i, error)
        }

        match turn_off_i_th_bit(&mut n, i) {
            Ok(_) => println!("Turn off {}-th bit: {}", i, n),
            Err(error) => println!("Error while turning off bit with index {}, error: {}", i, error)
        }
    }
}

fn turn_on_i_th_bit(x: &mut u64, i: u32) -> Result<(), String> {
    if i < 64 { 
        *x |= 1u64 << i;
        return Ok(());
    }

    Err(String::from("there is no bit with such index"))
}

fn turn_off_i_th_bit(x: &mut u64, i: u32) -> Result<(), String> {
    if i < 64 {
        *x &= ((1u64 << i) as u64).reverse_bits();
        return Ok(());
    }

    Err(String::from("there is no bit with such index"))
}
