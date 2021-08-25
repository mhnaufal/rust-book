#![allow(dead_code)]

/* 
* Rust doesn't care where we put the invoked function
* as long as we made it
*/
pub fn main_function() {
    parameter_function(313);

    /* Statement & Expression*/
    let x = 1;
    // let a = let b = 2;  // ERROR: because statement doesn't return a value so we can't use this

    let y = {
        x + 5
    };
    println!("Statement with expression: x = {}, y = {:?}", x, y);

    let _return_function = return_function(1);
    println!("Function with return: {}", _return_function);
}

/* The type of parameter in function must be declared specifically */
pub fn parameter_function(num: i32) {
    println!("The value of parameter inside parameter_function is: {}", num);
}

fn return_function(num: i32) -> i32 {
    num + 10
}