#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    pub fn creat(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    pub fn width(self) -> u32 {
        return self.width;
    }
    pub fn height(&self) -> u32 {
        return self.height;
    }
}

#[test]
fn test(){
    let a = Rectangle::new(12,15);
    let b = Rectangle::creat(12,15);
    println!("a is {:?}", a);
    println!("b is {:?}", b);
}