pub fn borrow() {
    /* Overview Borrowing / References
    * & = References value without taking the ownership
    *     so that we can use it in another function or expression
    * One mutable reference or any immutable reference
    * We can make immutable reference to become mutable reference after at least 'used' the immutable reference once
    * for example: we print out the value of immutable reference. It already count as 'use'
    */

    let s1 = String::from("Rustecean");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    /*** Mutable References ***/
    let mut s = String::from("hello");
    // let new_str = change( &mut s);
    println!("Mutable reference string = {:?}", change(&mut s));

    /* Only one mutable reference for a particular data at a time*/
    let mut mut_str = String::from("What the?");
    let str_2 = &mut mut_str;
    // let str_4 = &mut mut_str;   // ERROR: Because only one mutable reference for a particular data at a time
    println!("Mutable reference 2: {}", str_2);

    /* But we can do this because the mutable reference mut_str */
    /* already 'used' in println!(str_2) */
    let str_3 = &mut mut_str;
    println!("Mutable reference 3: {}", str_3);


    /* We can't combine immutable reference with mutable reference */
    let mut s = String::from("hello");
    let r1 = &s; // Immutable reference
    let r2 = &s; // Immutable reference
    // let r3 = &mut s; // ERROR: Because we already use s as a immutable reference in r1 and r2
    println!("2 Immutable reference :{}, {}", r1, r2);

    /*** Dangling References ***/
    /* a pointer that references a location in memory that may have been given to someone else, 
    * by freeing some memory while preserving a pointer to that memory 
    */
    let reference_to_nothing = not_dangle();
    println!("Dangle: {}", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

/* The dange_string goes out of scope and the value in heap already deleted */
/* But the reference still exist. So it's an ERROR */
// fn dangle() -> &String {
//     let dange_string = String::from("I am from dangle");
//     &dange_string
// }

/* The dange_string change ownership to whoever invoke this function  */
/* by get returned value from not_dangel() */
fn not_dangle() -> String {
    let dange_string = String::from("I am from dangle");
    dange_string
}