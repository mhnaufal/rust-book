pub fn ownership() {
    /* Overview Memory in Rust
     * Each value has 'owner'
     * Only one 'owner' at a time
     * When the 'owner' goes out of scope the value dropped (So we don't need to use free like in C/C++)
     * Copy trait = integer, boolean, float, char, tuple with copy trait element
     */

    /* String type
     * We don't know yet about the size at compile time, so it put on the heap
     * with pointer in the stack point to the located heap
     */
    let mut string = String::from("Hello");

    /* String literal
     * We already know the size at compile time, so it put on the stack
     */
    // let mut string = "Hello";   //ERROR if we try to insert a new string into it because it's fixed size
    println!("{:?}", string);

    string.push_str(" world!");
    println!("{:?}", string);

    /*** Move ***/
    /* Integer
    * Integer has 'Copy trait' so we can do copy value like below
    * x has 10 on it
    * y has x on it which equal to 10
    */
    let x = 10;
    let y = x;
    println!("Move x: {} and y: {}", x, y);

    /* String
    * String type doens't have 'Copy trait'
    * Shallow Copy = Copy the stack pointer, length, capacity but not the data on the heap
    */
    let s1 = String::from("hello");
    let _s2 = s1;    // s1 missing her ownership of string "hello" by 'moving' it to s2
    // println!("Move s1: {:?} and s2: {:?}", s1, s2); //ERROR because the value of s1 already moved ti s2 and the ownership change

    /* Clone
    * Deeply Copy = Copy the stack pointer, length, capacity, and the data on the heap
    */
    let s1 = String::from("hello");
    let s2 = s1.clone();    // s2 'deep copy' s1 and both are different
    println!("Move s1 = {}, s2 = {}", s1, s2);

    /*** Return value scope ***/
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    println!("s1 = {}", s1);

    let s2 = String::from("hello"); // s2 comes into scope
    println!("s2 = {}", s2);

    let s3 = takes_and_gives_back(s2.clone()); // s2 is moved into
                                               // takes_and_gives_back, which also
                                               // moves its return value into s3
    println!("s3 = {}", s3);
    println!("s2 new = {}", s2);
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello from give_ownership"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(mut a_string: String) -> String {
    // a_string comes into
    // scope
    a_string.push_str(" from take");
    a_string // a_string is returned and moves out to the calling function
}
