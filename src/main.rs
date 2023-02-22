#![allow(unused)]
mod error;
use std::fs;
use anyhow::anyhow;

fn main() {
    // read_dir();
    print1();
    // print_error();
}
// mod preclude;

fn print_error(){
    let err = anyhow!("error");
    println!("{:?}", err); // print error msg, context and backtrace
    // println!("{:?}", err); // print error msg, context and backtrace
    println!("{}", err.backtrace()); // print backtrace
}

fn print1() -> Result<(),error::Error>{
    println!("Try to check current dir");
    for entry in fs::read_dir("./")?.filter_map(|e| e.ok()){
        let entry = entry
        .path()
        .to_str()
        .map(String::from)
        .ok_or_else(|| error::Error::Generic(format!("Inval path")))?; // 对比一下加？与不加？的差别
        println!("{entry:?}");
    }
    Ok(())
}

// fn print2() -> Result<(),&'static str>{
//     println!("Try to check current dir");
//     for entry in fs::read_dir("./")?.filter_map(|e| e.ok()){
//         let entry = entry
//         .path()
//         .to_str()
//         .ok_or_else(|| Err("invalid path"))?; // 对比一下加？与不加？的差别
//         println!("{entry:?}");
//     }
//     Ok(())
// }