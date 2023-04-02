fn check_clone(){
    let v = vec![1, 2, 3];
    let sum = v.iter()
        .cloned()
        .filter(|x| x % 2 == 0)
        .fold(0, |sum, i| sum + i);
    let sum2=v.iter()
        .filter(|x|*x%2 == 0)
        .fold(0,|sum,i|sum+i);
    println!("{v:?}");
    println!("{sum}");
}

#[test]
fn test_clone(){
   check_clone();
}