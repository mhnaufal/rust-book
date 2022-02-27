#![allow(dead_code)]

// Note: x and y must be in the same type (T) when instantiate
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Note: x and y can be in the same type or different type 
#[derive(Debug)]
struct Point2<T, U> {
    x: U,
    y: T,
}

// TODO: method generic

pub fn generic() {
    let number_list = vec![34, 50, 69, 31, 2];

    let result = largest(&number_list);
    println!("The largest number is = {:?}", result);

    let char_list = vec!['w', 'e', 'e', 'd'];

    let result = largest(&char_list);
    println!("The largest char is = {}", result);

    /* Generic Struct */
    let integer_struct = Point { x: 1, y: 3 };
    println!("Integer point : {:?}", integer_struct);

    let diff_type = Point2 { x: 1.2, y: 4 };
    println!("Different point : {:?}", diff_type);
}

/* Generic & Trait */
fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
