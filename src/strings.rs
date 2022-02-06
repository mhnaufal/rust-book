#![allow(dead_code)]

pub fn strings() {
    /* Rust has 2 different strings (actually only 'str' the real one)
     * [1] String = a pointer with data on the heap, a bit slower than '&str'.
     *              It's growable, mutable, owned, UTF-8.
     * [2] &str = saved on the stack, faster than 'String'.
     */

    // create an empty string
    let phone = String::new();
    println!("Empty string = {:?}", phone);

    // give an initial data to string
    let mut laptop = "ASUS ".to_string();
    println!("to_string = {:?}", laptop);

    // we can also use ::from() function
    let pc = String::from("i12 Alder Lake 📟");
    println!("::from = {:?}", pc);

    // updating a String
    laptop.push_str("Zephyrus ");
    laptop.push('💻');
    println!("to_string = {:?}", laptop);

    // concatenate strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("Concat1 = {:?}", s);

    // more readable way to concat strings by using format!
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("Concat2 = {:?}", s);
    }

    /* Slicing
     * In Rust, we can't just get the index of a string.
     * We need get the range of index!
     */

    // let s1 = String::from("hello");
    // let h = s1[0];  // will result an ERROR

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("Slice = {:?}", s);
}
