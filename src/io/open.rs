#![allow(unused)]
use std::fs::File;
use std::io::{self, Read};
use std::env;

pub fn read_username_from_file() -> Result<String, io::Error> {
    // 打开文件，f是`Result<文件句柄,io::Error>`
    let f = File::open("hello.txt");

    let mut f = match f {
        // 打开文件成功，将file句柄赋值给f
        Ok(file) => file,
        // 打开文件失败，将错误返回(向上传播)
        Err(e) => return Err(e),
    };
    // 创建动态字符串s
    let mut s = String::new();
    // 从f文件句柄读取数据并写入s中
    match f.read_to_string(&mut s) {
        // 读取成功，返回Ok封装的字符串
        Ok(_) => Ok(s),
        // 将错误向上传播
        Err(e) => Err(e),
    }
}
fn read_username_from_file2() -> Result<String, io::Error> {
    let current_dir = env::current_dir().unwrap();
    println!("Current directory: {}", current_dir.display());
    let mut f = File::open("./src/io/test.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    println!("fuck {}", s); 
    Ok(s)
}

#[test]
fn test() {
    // read_username_from_file(); 
    // read_username_from_file().unwrap(); 
    let mut s = read_username_from_file2().unwrap_or(String::from("value faile"));
    // let mut s = read_username_from_file2().unwrap();
    println!("{s}")
}