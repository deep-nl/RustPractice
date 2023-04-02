use std::fmt;
#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8
}
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "\n{}\n{}",
            self.name, 
            self.age
        )
    }
}

// #[cfg(test)]
#[test]
fn test_print() {
    let i = 3.1415926;
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    let p = Person{name: "sunface".to_string(), age: 18};
    println!("--------------------------------------------------------------------------------");
    println!("{}", "-".repeat(80));
    println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);
    println!("--------------------------------------------------------------------------------");
    println!("{:#?}, {:#?}, {:#?}, {:#?}", i, s, v, p);
    println!("--------------------------------------------------------------------------------");
    // 如果v 和 p 不实现display就不能做如下打印，必须进行转换
    // convert a v to string
    let vv = v.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", ");
    // p = format!("{:?}", v);
    println!("{}, {}, {}, {}", i, s, vv, p);
    
}

// #[cfg(test)]
// mod tests {
// #[test]
// fn test_print() {
//     let i = 3.1415926;
//     let s = String::from("hello");
//     let v = vec![1, 2, 3];
//     let p = Person{name: "sunface".to_string(), age: 18};
//     println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);
//     print()
    
// }

// }