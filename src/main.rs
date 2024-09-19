use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use dashmap::DashMap;

fn main() {
    let hashmap = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for i in 0..10 {
        let hashmap = Arc::clone(&hashmap);
        let handle = thread::spawn(move || {
            let mut map = hashmap.lock().expect("Failed to lock the hashmap");
            map.insert(i, i * 10);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    let result = hashmap.lock().expect("Failed to lock the hashmap");
    println!("Результат с использованием Mutex и HashMap: {:?}", *result);

    let dashmap = Arc::new(DashMap::new());
    let mut handles = vec![];

    for i in 0..10 {
        let dashmap = Arc::clone(&dashmap);
        let handle = thread::spawn(move || {
            dashmap.insert(i, i * 10);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    println!("Результат с использованием DashMap: {:?}", dashmap);
}
