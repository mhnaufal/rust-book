pub fn control_flow() {
    let gpa: f64 = 3.9;

    if gpa >= 3.5 {
        println!("Cumlaude with GPA: {}", gpa);
    } else if gpa < 3.5 {
        println!("Nicely done: {}", gpa);
    } else {
        println!("Keep it up!: {}", gpa);
    }

    /* if inside let
     * Must be expression
     */
    let is_summa_cumlaude = if gpa == 4.0 { true } else { false };
    println!("Am I cumlaude? {:?}", is_summa_cumlaude);

    /* Loop */
    let mut counter = 0;
    let value = loop {
        counter += 1;

        if counter == 10 {
            break counter + 1;
        }
    };

    println!("Loop: {:?}", value);

    /* For */
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

    /* Reverse (polarity?) */
    for day in (1..5).rev() {
        println!("Number in backward {}", day);
    }

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

    println!("Temperature before: {} and after {}", temperature, new_temperature);

    // (2) Generate nth fibonacci number
    // 1 1 2 3 5 8 13 21 34
    let mut fibo_now: u32 = 1;
    let mut fibo_prev: u32 = 1;
    let mut counter = 1;

    let result = loop {
        counter += 1;

        if counter == 5 {
            break fibo_now
        }

        let temp = fibo_now;
        fibo_now = fibo_now + fibo_prev;
        fibo_prev = temp;
    };

    println!("Fibonacci: {}", result);
}
