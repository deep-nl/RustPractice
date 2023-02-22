#![allow(unused)]
use std::num::ParseIntError;


#[derive(Debug)]
struct SumError;

fn to_int(s: &str) -> i32 {
    // method1: 
    // s.parse().unwrap()
    // method2: 
    // s.parse().expect("wrong parse")
    // method3: 
    s.parse().unwrap_or(0)
}

// method4
fn to_int_1(s: &str) -> Option<i32> {
    s.parse().ok()
}

// method7
fn to_int_2(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}

fn sum_str_vec(strs: Vec<String>) -> String {
    let mut accum = 0i32;
    for s in strs {
        // accum += to_int(&s);
        // method 4-1
        accum += match to_int_1(&s){
           Some(val) => val,
           None => 0 
        };
        // method 4-2   how to use if let
        if let Some(val) = to_int_1(&s) {
            accum += val;
        };
        // method 4-3
        accum += to_int_1(&s).unwrap_or(0);
    }

    // if return is String
    return accum.to_string();
}

// method 5: return is Option<String>
fn sum_str_vec_1(strs: Vec<String>) -> Option<String> {
    let mut accum = 0i32;
    for s in strs {
        accum += to_int_1(&s)?;
    }
    Some(accum.to_string())
}

// method 6: return is Result
fn sum_str_vec_2(strs: Vec<String>) -> Result<String, SumError> {
    let mut accum = 0i32;
    for s in strs {
        accum += to_int_1(&s).ok_or(SumError)?;
    }
    Ok(accum.to_string())
}

// method 7: return is Result, remenber the error is ParseIntError
fn sum_str_vec_3(strs: Vec<String>) -> Result<String, ParseIntError> {
    let mut accum = 0i32;
    for s in strs {
        accum += to_int_2(&s)?;
    }
    Ok(accum.to_string())
}

// method 7-1: return is Result, if we want the error is not ParseIntError but some other certain error like SumError
fn sum_str_vec_4(strs: Vec<String>) -> Result<String, SumError> {
    let mut accum = 0i32;
    for s in strs {
        // TODO why need closure her,  check map_err params 
        accum += to_int_2(&s).map_err(|_| SumError)?;
        // unwrap_or_else
        // accum += to_int_2(&s).unwrap_or_else(op)?;
    }
    Ok(accum.to_string())
}

pub fn run() {
    let v = vec![String::from("3"),String::from("4")];
    let total = sum_str_vec(v); 
    println!("{:?}", total);

    let v = vec![String::from("abc"),String::from("4")];
    // let total = sum_str_vec(v);
    let total = sum_str_vec_1(v);
    println!("{:?}", total);
}




