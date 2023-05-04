#[test]
fn test_move() {
    let mut x = 42;

    let f = move || {
        println!("x: {}", x);
    };

    f();
    x += 1;

    f();
    println!("x: {}", x);
}

#[cfg(test)]
mod tests {
    use tokio::time::Duration;

    async fn sleep_then_print(timer: i32) {
        println!("Start timer {}.", timer);

        tokio::time::sleep(Duration::from_secs(1)).await;
    //                                            ^ execution can be paused here

        println!("Timer {} done.", timer);
    }

    async fn sleep_then_print_block(timer: i32) {
        println!("Start timer {}.", timer);
    
        // No .await here!
        std::thread::sleep(Duration::from_secs(1));
    
        println!("Timer {} done.", timer);
    }

    #[tokio::test]
    async fn test1() {
        // The join! macro lets you run multiple things concurrently.
        tokio::join!(
            sleep_then_print(1),
            sleep_then_print(2),
            sleep_then_print(3),
        );
        let blocking_task = tokio::task::spawn_blocking(|| {
            // This is running on a thread where blocking is fine.
            sleep_then_print(100);
        });
        sleep_then_print(4).await;
        sleep_then_print(5).await;
        sleep_then_print(6).await;
}
}