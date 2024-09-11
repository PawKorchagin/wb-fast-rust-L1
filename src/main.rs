use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    // Collect number of workers from CLI
    let args: Vec<String> = env::args().collect();
    let num_workers: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(10) // default: 4 workers
    } else {
        10
    };

    // Creates channel (mpsc = multi-producer, single-consumer)
    let (tx, rx) = mpsc::channel();

    // Wraps up receiver into Arc<Mutex<Receiver<()>>>
    let rx = Arc::new(Mutex::new(rx));

    // Executes workers
    for i in 0..num_workers {
        let rx = Arc::clone(&rx); // Clones Arc for every worker
        thread::spawn(move || {
            loop { // Infinite loop
                // Blocks access to receiver and tries to get data
                let data = rx.lock().unwrap().recv();
                match data {
                    Ok(value) => {
                        println!("Worker {} received: {}", i + 1, value);
                    }
                    Err(_) => {
                        println!("Worker {}: Channel closed", i + 1);
                        break;
                    }
                }
            }
        });
    }

    // Main thread send data into channel
    let tx_main = tx.clone(); // Clones sender
    thread::spawn(move || {
        let mut count = 0;
        loop { // Infinite loop
            count += 1;
            // Send counter into queue
            tx_main.send(count).unwrap();
            thread::sleep(Duration::from_millis(500)); // Just pause for clarity
        }
    });

    // Waits for interruption
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
