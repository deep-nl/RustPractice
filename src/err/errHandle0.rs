#![allow(unused)]
use std::fs;
use std::io::{self, Read};
use std::error::Error;
use eyre::Result;

fn read_username_from_file0(path: &str) -> Result<String, io::Error> {
    let mut f = fs::File::open(path)?;
    // let ff = &f;
    // let mut q = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s) 
}

fn read_file1() -> Result<(),Box<dyn Error>>{
    let f = fs::File::open("test.txt");
    let f = match f {
       Ok(file) => file,
       Err(e) => {
            println!("{e}");
            return Err(e.into());
        }
    };
    Ok(())
}

fn read_file2() -> Result<(),Box<dyn Error>>{
    let f = fs::File::open("test.txt");
    Ok(())
}

fn last_char_of_the_line(text: &str) -> Option<char>{
    text.lines().next()?.chars().last()
}