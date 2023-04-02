use tracing::Level;
use tracing_subscriber::FmtSubscriber;
fn f() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::error!("error message");
    tracing::info!("info message");
}

#[test]
fn test_f(){
    f()
}