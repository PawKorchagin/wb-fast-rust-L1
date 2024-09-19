fn quicksort_std(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return; // Base case: arrays of size 0 or 1 are already sorted
    }

    // Use the built-in sort method on the slice, which is highly optimized
    arr.sort_unstable(); // Order stability isn't needed
}

fn main() {
    let mut array = [3, 6, 8, 10, 1, 2, 1, 5, 7, 9];
    println!("Original array: {:?}", array);

    array.sort_unstable();

    println!("Sorted array: {array:?}");
}
