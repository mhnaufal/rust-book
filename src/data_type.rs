#![allow(dead_code)]

pub fn data_type() {
    /* Type annotation
     * To convert from one type to another type.
     * Similiar to cast except it also check whether the type can be cast or not.
     */
    let string_to_num: i32 = "23".parse().expect("Not a number!");
    println!("String to num: {}", string_to_num);

    println!("----------------------------------------------------");

    /*** Integer ***/
    /* '_' for visual separator
     */
    let decimal = 100_000;
    println! {"Decimal: {}", decimal};

    /*** Hex ***/
    let hex = 0xF;
    println!("Hex: {}", hex);

    /*** Octal ***/
    let octal = 0o11;
    println!("Octal: {}", octal);

    /*** Binary ***/
    let binary = b'1';
    println!("Binary: {}", binary);

    println!("----------------------------------------------------");

    /*** Integer overflow ***/
    let overflow: u8 = 255;
    println!("Not yet overflowed: {}", overflow);

    // ERROR: overflowed
    // overflow = 257;
    // println!("Overflowed: {}", overflow);

    /*** Float ***/
    /* Default float is f64
     */
    let float_1 = 1.5;
    let float_2: f32 = 2.5;
    println!("Floats: {} and {}", float_1, float_2);

    /*** Boolean ***/
    let success: bool = true;
    println!("Boolean: {}", success);

    /* Character
     * char use single quotes ' '
     */
    let crab: char = '🦀';
    println!("Char: {}", crab);

    println!("----------------------------------------------------");

    /* Tuple
     * Fixed size length
     * Elements inside it can have different type
     */
    let tuple = (100, 202.2, '🐧');
    println!("1st Tuple: {:?}", tuple);

    let tuple_2: (f64, char, &str) = (1.0, 'F', "What?");
    println!("2nd Tuple: {:?}", tuple_2);

    /* Destructoring tuple */
    let (x, y, z) = tuple;
    println!("Destructor: {}, {}, {}", x, y, z);

    /* Dot operator
     * To access
     */
    let x = tuple_2.2;
    println!("Dot operator: {}", x);

    println!("----------------------------------------------------");

    /* Array
     * Fixed length
     * All elements have the 'same type'
     * Data saved inside stack it means that the type must be known at compile.
     */
    let array = [1, 2, 3];
    println!("1st Array: {:?}", array);

    let array_2: [f64; 2] = [1.2, 2.1]; // create an array with 2 elements of type f64
    println!("2nd Array: {:?}", array_2);

    let array_3: [i32; 3] = [13; 3]; // create an array with 3 elements of number '13'
    println!("3rd Array: {:?}", array_3);
}
