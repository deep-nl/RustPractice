use std::env;

fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("Current directory: {:?}", current_dir);

    Ok(())
}
