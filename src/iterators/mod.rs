pub mod iterator1;

#[cfg(test)]
mod tests {
    #[test]
    fn cloned() {
        let a = [1, 2, 3];

        let v_cloned: Vec<i32> = a.iter().cloned().collect();
        let v_iter: Vec<&i32> = a.iter().collect();
        let v_clone: Vec<&i32> = a.clone().iter().collect();
        let v_clone_into: Vec<i32> = a.clone().into_iter().collect();
        // cloned is the same as .map(|&x| x), for integers
        let v_map: Vec<_> = a.iter().map(|&x| x).collect();

        println!("{a:?}");
        // println!("{v_clone:?}");

        println!("{:p}", a.as_ptr());
        // println!("{:p}", v_clone.as_ptr()); // error: temporary value dropped while borrowed
        println!("{:p}", v_iter.as_ptr());
        println!("{:p}", v_clone_into.as_ptr());
        
        assert_eq!(v_cloned, vec![1, 2, 3]);
        assert_eq!(v_map, vec![1, 2, 3]);
    }

    #[test]
    fn filter(){
        let a = [0i32, 1, 2];
        let b = [0, 2, 3];
    
        // Because the closure passed to filter() takes a reference, and many iterators iterate over references, 
        // this leads to a possibly confusing situation, where the type of the closure is a double reference:
        let mut filter_iter = a.iter().filter(|x| **x > 1); // need two *s!
        // It’s common to instead use destructuring on the argument to strip away one:
        let mut filter_iter2 = a.iter().filter(|&x| *x > 1); // both & and *
        let mut filter_iter3 = a.iter().filter(|&&x| x > 1); // both & and *

        // funny
        let mut filter_iter4 = a.iter().filter(|x| x.is_positive()); // 
        // let mut filter_iter5 = b.iter().filter(|x| x.is_positive()); // error: why
        // compare with map
        let mut map_iter = b.iter().map(|x| 2 * x); 
    
        assert_eq!(filter_iter.next(), Some(&2));
        assert_eq!(filter_iter.next(), None);
        assert_eq!(map_iter.next(), Some(2));
        assert_eq!(map_iter.next(), Some(4));
        assert_eq!(map_iter.next(), Some(6));
        assert_eq!(map_iter.next(), None);
    }

    #[test]
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
    fn find() {
        let a = [1, 2, 3];

        assert_eq!(a.iter().find(|&&x| x == 2), Some(&2));
        assert_eq!(a.iter().find(|&&x| x == 5), None);

        print!("{a:?}\n")
    }

}