fn test_move() {
    let mut x = 42;

    let f = move || {
        println!("x: {}", x);
    };

    x += 1;

    f();
}
