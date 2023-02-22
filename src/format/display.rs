// 在 Rust 中，无法直接为外部类型实现外部特征，但是可以使用newtype解决此问题
use std::fmt;
struct Array(Vec<i32>);
impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "数组是：{:?}", self.0)
    }
}

fn main() {
    let arr = Array(vec![1, 2, 3]);
    println!("{}", arr);
}
