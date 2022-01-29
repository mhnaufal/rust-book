#![allow(dead_code)]

extern crate digits_iterator;
use std::convert::TryInto;

use digits_iterator::*;

pub fn is_armstrong_number(num: u32) -> bool {
    let mut total: u8 = 0;
    let count = num.digits().count();

    /* Version 1 */
    // let digits: Vec<_> = num
    //     .digits()
    //     .enumerate()
    //     .map(|(_, x)| (total as usize) + (x as usize) ^ count)
    //     .collect();
    // println!("digits = {:?} ---- total = {:?}", digits, total);

    /* Version 2 */
    for (_pos, digit) in num.digits().enumerate() {
        total = total + (digit.pow(count.try_into().unwrap()));
    }

    let ans = total == (num as u8);
    if ans {
        println!("{}", ans);
        return true;
    }

    println!("{}", ans);
    return false;
}
