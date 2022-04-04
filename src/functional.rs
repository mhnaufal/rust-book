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
}