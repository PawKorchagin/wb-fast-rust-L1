use flume::{Receiver, Sender};
use std::time::Duration;
use tokio::signal;
use tokio::task;
use tokio::time;

#[tokio::main]
async fn main() {
    // Creates multi-thread channel
    let (tx, rx): (Sender<u32>, Receiver<u32>) = flume::unbounded();

    // Gets number of workers from CLI
    let args: Vec<String> = std::env::args().collect();
    let num_workers: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(10) // Default: 10
    } else {
        10
    };

    // Executes workers
    for i in 0..num_workers {
        let rx = rx.clone(); // Clone receiver for every worker
        task::spawn(worker_task(rx, i + 1)); 
    }

    // Executes task to gen data
    let tx_clone = tx.clone(); // Clone sender
    task::spawn(async move {
        let mut count = 0;
        loop {
            count += 1;
            if tx_clone.send_async(count).await.is_err() {
                break;
            }
            time::sleep(Duration::from_millis(500)).await; // Pause for clarity
        }
    });

    // Handles shutdown signal (Ctrl+C)
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("Ctrl+C received, shutting down...");

    // Sends signal to shutdown all workers
    for _ in 0..num_workers {
        tx.send_async(0).await.expect("Failed to send shutdown signal");
    }

    // Pause to shutdown all workers safely 
    time::sleep(Duration::from_secs(1)).await;
}

// Task for each worker
async fn worker_task(rx: Receiver<u32>, worker_id: usize) {
    loop {
        match rx.recv_async().await {
            Ok(data) => {
                if data == 0 {
                    println!("Worker {} shutting down...", worker_id);
                    break;
                }
                println!("Worker {} received: {}", worker_id, data);
            }
            Err(_) => {
                println!("Worker {}: Channel closed", worker_id);
                break;
            }
        }
    }
}
