use tokio::task;

fn fib(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib(n-1) + fib(n-2)
    }
}

#[tokio::test]
async fn test_task() {
    let a = task::spawn_blocking(|| {
        log::info!("start cal");
        let ret = fib(100);
        log::info!("end cal, result is {:?}", ret);
    });

    let b = task::spawn_blocking(|| {
        log::info!("start cal");
        let ret = fib(99);
        log::info!("end cal, result is {:?}", ret);
    });

    tokio::join!(a,b).0.unwrap();

}