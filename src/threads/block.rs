use std::time::{Duration, Instant};
use tokio::time::{sleep, Instant as TokioInstant};

fn block() {
    // Blocking sleep
    let start = Instant::now();
    std::thread::sleep(Duration::from_secs(1));
    println!("Blocking sleep: {:?}", start.elapsed());

    // Non-blocking sleep
    let start = Instant::now();
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        sleep(Duration::from_secs(1)).await;
        println!("Non-blocking sleep: {:?}", start.elapsed());
    });

    // Non-blocking sleep with Tokio's Instant
    let start = Instant::now();
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let now = TokioInstant::now();
        sleep(Duration::from_secs(1)).await;
        println!("Non-blocking sleep with Tokio: {:?}", start.elapsed() + now.elapsed());
    });
}
