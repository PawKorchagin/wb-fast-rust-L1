use std::{io, thread};

use std::sync::mpsc;

fn main() {
    // Prepare a blank string to store user input
    let mut input = String::new();

    // Read a line from stdin (standard input) and panic on error
    io::stdin().read_line(&mut input).unwrap();

    // Convert the input string to a usize using parse<T>() -> T
    // Panic if the conversion fails
    let n: usize = input.trim().parse().unwrap(); 
    // Now n is a usize and can be used as the vector size

    // Create a Vec<u64> with an increasing sequence from 1 to n
    // For example, if n = 3: [1, 2, 3]
    let numbers: Vec<u64> = (1..=n as u64).collect();
    // Uses the range (1..=n as u64) to create an iterator
    // Uses .collect() to convert the iterator into a vector

    // Closure (anonymous function) that takes ownership of x
    // Returns the square of x as an Option<u64>:
    // 1. None if there is a u64 overflow
    // 2. The result of the multiplication otherwise
    let make_square = move |x: u64| -> Option<u64> { x.checked_mul(x) };

    // Creates streaming channel
    let (_sender, _reciever) = mpsc::channel::<u64>();

    // Create a thread handler to manage parallel execution
    // Takes ownership of variables by moving them into the thread
    let handler = thread::spawn(move || {
        // All code here runs in a separate thread
        for i in 0..n {
            // Handle the optional result from make_square
            if let Some(x) = make_square(numbers[i]) {
                // sends x to channel queue
                _sender.send(x).unwrap();
            }
        }
    });

    let mut res = 0u128;
    for _ in 0..n {
        // receives value from queue and adds to res
        res += _reciever.recv().unwrap() as u128;
    }

    println!("{res}");

    // Wait for the spawned thread to finish
    handler.join().unwrap();

}
