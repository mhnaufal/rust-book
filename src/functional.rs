#![allow(dead_code)]
#![allow(unused_assignments)]

/* Functional Programming
* Closure
* Iterators
*/

use std::{thread, time::Duration};

pub fn functional() {
    let expensive_closure = |num: u32| {
        println!("Calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    println!("Expensive closure: {:#?}", expensive_closure(3));

    /* Catch outside block variable using closure
     * we can't do this using function
     * we don't have to explicitly tell the type of parameters in closure, as long as it's we inferred it when we call the closure
     */
    let out = 13;

    let is_eq_out = |mtch| mtch == out;

    assert!(is_eq_out(13));

    /* Iterators */
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // Iterator implement trait with `next` method to implement
    #[test]
    fn iterator_demo() {
        let v1 = vec![3, 2, 1];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&3));   // get the next about of the iterator
        assert_eq!(Some(&2), v1_iter.next());
    }
}
