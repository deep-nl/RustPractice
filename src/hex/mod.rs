#[cfg(test)]
mod tests {
    // use std::
    use hex;
    fn helper(num:i32) ->String{
        let hex_string = format!("{:x}", num);
        hex_string
    }
    #[test]
    fn cmp() {
        let a:i32 = 2;     // Bit presentation 10
        let b:i32 = 3;     // Bit presentation 11
     
        let mut result:i32;
     
        result = a & b;
        println!("(a & b) => {:#06x} ",result);
     
        result = a | b;
        println!("(a | b) => {} ",result) ;
     
        result = a ^ b;
        println!("(a ^ b) => {} ",result);
     
        result = !b;
        println!("(!b) => {} ",result);
     
        result = a << b;
        println!("(a << b) => {}",result);
     
        result = a >> b;
        println!("(a >> b) => {}",result);
    }
}

#[test]
fn cmp1() {
    let a:i64 = 10202020999999999;     // Bit presentation 10
 
    let mut result:i64;
 
    result = a;
    println!("(a & b) => {:#32x} ",result);
}