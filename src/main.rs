use reqwest;
use std::fs::File;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::sync::Semaphore;
use tokio::task;

async fn send_get_request(url: &str, counter: Arc<Mutex<u64>>, sem: Arc<Semaphore>) {
    let _permit = sem.acquire().await;

    let client = reqwest::Client::new();
    if let Ok(_) = client.get(url).send().await {
        let mut count = counter.lock().unwrap();
        *count += 1;
    }

    // The _permit will be dropped here, releasing the semaphore.
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Enter the URL:");
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Failed to read line");
    let url = url.trim();

    let counter = Arc::new(Mutex::new(0u64));

    // Let's allow up to 1000 concurrent requests. You can adjust this value.
    let sem = Arc::new(Semaphore::new(1000));

    let start_time = Instant::now();
    for _ in 0..100_000 {  // Adjust as needed for the total number of requests
        let url_clone = url.to_string();
        let counter_clone = counter.clone();
        let sem_clone = sem.clone();

        task::spawn(async move {
            send_get_request(&url_clone, counter_clone, sem_clone).await;
        });
    }

    // We wait for all tasks to complete before moving on.
    while sem.available_permits() != 1000 {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    let total_requests = *counter.lock().unwrap();
    let elapsed_time = start_time.elapsed();

    // Write the results to a text file
    let mut file = File::create("report.txt").unwrap();
    writeln!(file, "Total requests sent: {}", total_requests).unwrap();
    writeln!(file, "Total time taken: {:?}", elapsed_time).unwrap();
}
