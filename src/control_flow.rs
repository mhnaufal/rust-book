#![allow(dead_code)]

pub fn control_flow() {
    let gpa: f64 = 3.9;

    if gpa >= 3.5 {
        println!("Cumlaude with GPA: {}", gpa);
    } else if gpa < 3.5 {
        println!("Nicely done: {}", gpa);
    } else {
        println!("Keep it up!: {}", gpa);
    }

    /* 'if' inside let
     * must be an expression
     */
    let is_summa_cumlaude = if gpa == 4.0 { true } else { false };
    println!("Am I cumlaude? {:?}", is_summa_cumlaude);

    /* 'Loop' loop */
    let mut counter = 0;
    let value = loop {
        counter += 1;

        if counter == 10 {
            break counter + 1;
        }
    };

    println!("Loop occured: {:?}", value);

    println!("----------------------------------------------------");

    /* 'For' loop */
    let days = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    for day in days.iter() {
        println!("Today is {}", day);
    }

    println!("----------------------------------------------------");

    /* Reverse (polarity?) */
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    for day in arr.iter().rev() {
        println!("Reverse iteration --> {}", day);
    }

    println!("----------------------------------------------------");

    /* Break
     * you can return value from a break statement
     */
    let mut value = 0;
    let new_value = loop {
        value += 1;

        print!("{} ", value);

        if value * 3 == 9 {
            break value; // set the value of variable new_value to 'value' from break statement
        }
    };
    println!("\nNew value: {}", new_value);

    println!("----------------------------------------------------");

    /* Challenge */
    // (1) Convert temperature
    let temperature = 20;
    let r#type: char = 'C';

    let new_temperature = if r#type == 'C' || r#type == 'c' {
        (temperature * 5) / 9
    } else if r#type == 'F' || r#type == 'f' {
        ((temperature - 32) * 9) / 5
    } else {
        temperature
    };

    println!(
        "[1] Temperature before: {} and after: {}",
        temperature, new_temperature
    );

    // (2) Generate nth fibonacci number
    // 1 1 2 3 5 8 13 21 34
    let mut fibo_now: u32 = 1;
    let mut fibo_prev: u32 = 1;
    let mut counter = 1;

    let result = loop {
        counter += 1;

        if counter == 5 {
            break fibo_now;
        }

        let temp = fibo_now;
        fibo_now = fibo_now + fibo_prev;
        fibo_prev = temp;
    };

    println!("[2] Fibonacci: {}", result);
}
