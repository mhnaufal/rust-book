#![allow(dead_code)]

pub fn variable () {
    // Variables are immutable by default
    let nim = 240601;
    println!("Your NIM is: {}", &nim);

    // ERROR - because reassign immutable variable
    // nim = 240602;

    // Not cause error because it's a 'shadowing' technique 
    // Create a new variable with same name but different type
    let nim = 240602.1;
    println!("Your new NIM is: {}", &nim);

    let mut gpa: f32 = 3.8;
    println!("Your GPA is: {}", gpa);

    gpa = 4.0;
    println!("Your new GPA is: {}", &gpa);

    // const are always immutable
    // and we need to define the type manually
    const MAX_GPA: f64 = 4.0;
    println!("Maximum GPA is: {}", MAX_GPA);
}