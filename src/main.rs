use std::sync::{Arc, Mutex};
use std::thread;

struct Counter {
    count: Arc<Mutex<i32>>
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: Arc::new(Mutex::new(0)), // Initialize with 0
        }
    }

    fn increment(&self) {
        let mut counter = self.count.lock().unwrap();
        *counter += 1;
    }

    fn get_value(&self) -> i32 {
        let counter = self.count.lock().unwrap();
        *counter
    }
}

fn main() {
    let counter = Counter::new();
    let mut handles = vec![];

    // Threads spawning
    for _ in 0..10 {
        let counter_clone = Counter {
            count: Arc::clone(&counter.count), // Clone the Arc to share ownership between threads
        };

        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.increment();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter.get_value());
}
