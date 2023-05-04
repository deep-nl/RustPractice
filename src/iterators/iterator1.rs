// pub mod study;
#![allow(unused)]

fn check(){
    let results = [Ok(1), Err("nope"), Ok(3), Err("bad")];
    // let results = [Ok(1), Ok(3), Ok(4)];

    let result: Result<Vec<_>, &str> = results.iter().cloned().collect();

    // gives us the first error
    assert_eq!(Err("nope"), result);
    assert_eq!(Err("nope"), result);
    println!("{:?}", result);
}

fn cloned() {
    let a = [1, 2, 3];

    let v_cloned: Vec<i32> = a.iter().cloned().collect();
    
    // cloned is the same as .map(|&x| x), for integers
    // let v_map: Vec<_> = a.iter().map(|&x| x).collect();
    
    assert_eq!(v_cloned, vec![1, 2, 3]);
    // assert_eq!(v_map, vec![1, 2, 3]);
}

fn filter(){
    let a = [0, 1, 2];
    let b = [1, 2, 3];

    // Because the closure passed to filter() takes a reference, and many iterators iterate over references, 
    // this leads to a possibly confusing situation, where the type of the closure is a double reference:
    let mut filter_iter = a.iter().filter(|x| **x > 1); // need two *s!
    // It’s common to instead use destructuring on the argument to strip away one:
    let mut filter_iter2 = a.iter().filter(|&x| *x > 1); // both & and *
    let mut filter_iter3 = a.iter().filter(|&&x| x > 1); // both & and *
    // compare with map
    let mut map_iter = b.iter().map(|x| 2 * x); 

    assert_eq!(filter_iter.next(), Some(&2));
    assert_eq!(filter_iter.next(), None);
    assert_eq!(map_iter.next(), Some(2));
    assert_eq!(map_iter.next(), Some(4));
    assert_eq!(map_iter.next(), Some(6));
    assert_eq!(map_iter.next(), None);
}

fn filter2(){
    let a = [0i32, 1, 2];

    let mut iter = a.iter().filter(|x| x.is_positive());

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
}

fn inspect(){
    let a = [1, 4, 2, 3];

    let sum_ = a.iter()
        .clone()
        .filter(|x|*x%2 ==0)
        .fold(0, |sum,i|sum+i);

    // this iterator sequence is complex.
    let sum = a.iter()
        .cloned()
        .filter(|x| x % 2 == 0)
        .fold(0, |sum, i| sum + i);

    println!("{sum}");

    // let's add some inspect() calls to investigate what's happening
    let sum = a.iter()
        .cloned()
        .inspect(|x| println!("about to filter: {x}"))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("made it through filter: {x}"))
        .fold(0, |sum, i| sum + i);

    println!("{sum}");
}

#[test]
fn test_inspect(){
    inspect()
}
