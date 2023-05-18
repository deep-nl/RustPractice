use tokio::time;
use tokio::io::{AsyncRead, AsyncReadExt};
use anyhow::anyhow;
use log::Level;

async fn sleeper() {
    log::info!("sleeping");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("awake!");
}

async fn reader() {
    log::info!("Start reading");
    let mut f = tokio::fs::File::open("input.txt").await.unwrap();
    let mut content = vec![];
    f.read_to_end(&mut content).await.unwrap();
    log::info!("End reading, length: {}", content.len());

    // This will decrease the speed
    fib(100);
    // use this way to enhance performance
    tokio::task::spawn_blocking(move || {
        log::info!("start cal");
        fib(100);
        log::info!("end cal");
    }).await.unwrap();
}

// This a cpu process
fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n-1) + fib(n-2)
    }
}

async fn run() {
    sleeper().await;
    reader().await;
}

async fn async_run() {
    tokio::join!(
        sleeper(),
        reader(),
    );
}

#[tokio::test]
// async fn test_run() -> anyhow::Result<()> {
async fn test_run() {
    simple_logger::init_with_level(Level::Info).unwrap();
    let start = std::time::Instant::now();
    // run().await;
    async_run().await;
    let end = std::time::Instant::now();
    log::info!("time use {:?}", end - start);
    // Ok(());
}