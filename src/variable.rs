#![allow(dead_code)]

pub fn variable() {
    /* Imutable variable
     * Variables are immutable by default
     */
    let nim = 240601191;
    println!("Your current NIM is: {}", &nim);

    // ERROR - because reassign immutable variable
    // nim = 240602;

    /* Shadowing
     * Not cause error because it's a 'shadowing' technique
     * Create a 'new variable' with 'same name' but 'different data type'
     */
    let nim = 240602.1;
    println!("Your new NIM is: {}", &nim);

    println!("----------------------------------------------------");

    /* However, how do we get the first nim variable? */
    /* The answer is by using block */
    let weight = 80;
    println!("Your weight is: {}", weight); // will result 80
    {
        let weight = 80.5;
        println!("Your weight is: {}", weight); // will result 80.5
    }
    println!("Your weight is: {}", weight); // will result 80

    println!("----------------------------------------------------");

    /* Casting data type */
    let mut gpa: f32 = 3.8;
    println!("Your current GPA is: {}", gpa);

    gpa = (gpa as i64) as f32;
    println!("Your new GPA is: {}", &gpa);

    /* Constant
     * const are always immutable
     * and we need to define the type manually
     */
    const MAX_GPA: f64 = 4.0;
    println!("Maximum GPA is: {}", MAX_GPA);

    /* Variable expression */
    let calculate = {
        let first_number = 1;
        let second_number = 2;
        let first_number = first_number + 3;
        let first_number = first_number + second_number;
        first_number
    };

    println!("The result = {}", calculate);
}
