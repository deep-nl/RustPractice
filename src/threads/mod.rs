#[test]
fn test_move() {
    let mut x = 42;

    let f = move || {
        println!("x: {}", x);
    };

    f();
    x += 1;

    f();
    println!("x: {}", x);
}