use std::time::Duration;
use tokio::time::sleep;
use tokio::task::JoinHandle;
use rand::Rng;

async fn simulate_http_request(id: u32) -> String {
    // let n = rng.
    println!("Sending request {}", id);
    // Simulate some work by delaying for 100ms
    sleep(Duration::from_millis(1000)).await;
    println!("Received response for request {}", id);
    format!("Response for request {}", id)
}

#[tokio::test]
async fn test() {
    // Spawn a new task for each request and collect the JoinHandles
    let handles = (0..10)
        .map(|id| tokio::spawn(async move { simulate_http_request(id).await }))
        .collect::<Vec<_>>();

    // blocking wait
    for i in handles {
        i.await;
    }

    // Wait for all tasks to complete and print the results
    // unblocking wait
    // let results = futures::future::join_all(handles).await;
    // println!("Results: {:?}", results);
}
