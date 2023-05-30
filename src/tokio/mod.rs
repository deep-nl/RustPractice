pub mod http;
pub mod block1;
pub mod task;

// https://www.youtube.com/watch?v=dOzrO40jgbU


#[cfg(test)]
mod tests {
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::{Duration, Instant};
    use rand::prelude::*;

    use tokio::runtime;
    use tokio::time::sleep;
    use futures::future::*;

    #[test]
    fn thread_tasks() {
        let thread_handles: Vec<JoinHandle<_>>= 
            (1..=100)
            .map(|i| {
                // 1-2seconds delay
                let delay = rand::thread_rng().gen_range(1000..2000);

                // name thread
                let builder = thread::Builder::new().name(format!("custom-{}",i));

                //spawn
                builder.spawn(move || {
                    thread::sleep(Duration::from_millis(delay));

                    println!("Delay {} ms done! Thread name: {}",
                        delay,
                        thread::current().name().unwrap()
                    )
                // Dont use ; here! Because If you want the last value of a function (be it a top-level function or a closure) to be returned implicitly, you should not end it with a semicolon. Semicolons terminate statements that are being run for effects, not the final expression of a returning function. Replace
                }).unwrap()
            }).collect();
        
        // wait for finish
        for h in thread_handles {
            let _ = h.join();
        }
        println!("Jobs all done!")
    }

    #[test]
    fn tokio_tasks() {
        let mut rt = runtime::Builder::new_current_thread()
            .worker_threads(4)
            .enable_time()
            .build()
            .unwrap();

        for _ in 1..100 {
            // 1-2 second delay
            let delay = rand::thread_rng().gen_range(1000..2000);

            rt.block_on(async move {
                sleep(Duration::from_millis(delay)).await;
                println!(
                    "Delay {} ms done! Thread name: {:?}",
                    delay,
                    thread::current().name()
                );
            });
        };

        // rt.shutdown_background()
        println!("Jobs all done!")


    }
}