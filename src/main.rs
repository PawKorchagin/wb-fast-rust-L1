use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx1, rx2) = mpsc::channel();
    let (tx2, rx3) = mpsc::channel();

    let mut handles = Vec::new();

    let handle_square = thread::spawn(move || {
        for num in rx2 {
            tx2.send(num * num).unwrap();
        }
    });

    handles.push(handle_square);

    let handle_print = thread::spawn(move || {
        for square in rx3 {
            println!("{}", square);
        }
    });

    handles.push(handle_print);

    const N: usize = 10;
    let mut numbers: [u32; N] = [0; N];

    for i in 0..N {
        numbers[i] = (i + 1) as u32;
    }

    for &number in &numbers {
        tx1.send(number).unwrap();
    }
    drop(tx1); 

    for handle in handles {
        handle.join().unwrap();
    }
}
