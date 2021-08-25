#![allow(dead_code)]

pub fn vector() {
    // Create an empty vector
    let vec1: Vec<i32> = Vec::new();
    println!("{:?}", &vec1);

    // Create a vector using Rust macro vector
    let mut vec2 = vec![3301, 619, 333];
    println!("{:?}", &vec2);

    // Add element into a vector
    vec2.push(313);
    println!("{:?}", &vec2);

    // Get an element from a vector
    let second = &vec2[1];
    println!("{}", second);

    // Use match instead of normal accessing to prevent compiler panic when index out of range
    match vec2.get(1) {
        Some(t) => println!("The second element is {}", t),
        None => println!("There is no second element"),
    }

    // vec2.push(13);   // ERROR mutable borrow in the same scope
    println!("{:?}", second);
    println!("{:?}", vec2);

    // Iterate through a vector
    // We need add the '&' in front of vec2 so that it treat as borrowing 
    // not moving. If we not give the '&' the vec2 will move ownership
    // into for loop scope and at the end it will be destroy
    for v in &vec2 {
        println!("{}", v);
    }

    // that's why we get an error here, if we not add the '&'
    // because the vec2 already missing in this function scope
    // when get out from for loop scope
    for v in &mut vec2{
        *v += 5;
        println!("{}", v);
    }

    // Remove first element from vector
    vec2.pop();
    println!("{:?}", vec2);
    
}
