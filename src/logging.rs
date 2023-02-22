#![allow(unused)]
use env_logger;
use log::{debug, error, log_enabled, info, Level, LevelFilter};
use std::io::Write;
use chrono::Local;

fn logger() {
    // 注意，env_logger 必须尽可能早的初始化
    // env_logger::init();
    env_logger::Builder::new()
    .format(|buf, record| {
        writeln!(buf,
            "{} [{}] - {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.args()
        )
    })
    .filter(None, LevelFilter::Info)
    .init();
    let a = 10;

    debug!("this is a debug {}", "message");
    // error!("this is printed by default");
    for i in 1..a{
        //
        info!("the number is {}", i)
    }
    info!("a is {}", a);
    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}