use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};
use tokio;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    // V1
    // using channels

    let (tx, rx) = mpsc::channel();
    let worker_thread = thread::spawn(move || {
        while let Ok(message) = rx.recv() {
            println!("Thread received: {}", message);
        }
        println!("Channel closed, stopping thread.");
    });

    tx.send("Message 1".to_string()).expect("Failed to send message 1");
    tx.send("Message 2".to_string()).expect("Failed to send message 2");
    
    drop(tx);
    if let Err(e) = worker_thread.join() {
        eprintln!("Worker thread panicked: {:?}", e);
    }

    // V2
    // Stopping Tokio tasks using CancellationToken
    let token = CancellationToken::new();
    let token_clone = token.clone();

    let tokio_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = token_clone.cancelled() => {
                    println!("CancellationToken received, stopping Tokio task.");
                    break;
                }
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    println!("Tokio task running...");
                }
            }
        }
    });

    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Cancel the task
    token.cancel();
    tokio_task.await.expect("Failed to await tokio task");

    println!("All tasks ended");
}
