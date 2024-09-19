use std::io;
use tokio::time::{self, Duration};
use flume::{Receiver, Sender};

#[tokio::main]
async fn main() {
    // Creates channel
    let (tx, rx): (Sender<u32>, Receiver<u32>) = flume::unbounded();

    // Reads n from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.parse().unwrap();

    // Duration of program running (n secs)
    let duration = Duration::from_secs(n);

    // Executes task to send data in channel
    tokio::spawn(async move {
        let mut count = 0;
        loop {
            count += 1;
            if tx.send_async(count).await.is_err() {
                println!("The error occured while send data");
                break;
            }
            time::sleep(Duration::from_millis(500)).await; // Pause between sendings
        }
    });

    // Executes task to 
    tokio::spawn(async move {
        while let Ok(data) = rx.recv_async().await {
            println!("Получено: {}", data);
        }
    });

    // Waiting before shutting down
    time::sleep(duration).await;

    println!("The time is over, shutting down...");
}
