pub fn data_type() {
    // Type annotation - TO convert from one type to another type
    let string_to_num: i32 = "23".parse().expect("Not a number!");
    println!("String to num: {}", string_to_num);

    /* Decimal
    * '_' for visual separator
    */
    let decimal = 100_000;
    println!{"Decimal: {}", decimal};

    /*** Hex ***/
    let hex = 0xF;
    println!("Hex: {}", hex);

    /*** Octal ***/
    let octal = 0o11;
    println!("Octal: {}", octal);

    /*** Binary ***/
    let binary = b'1';
    println!("Binary: {}", binary);

    /*** Integer overflow ***/
    let overflow: u8 = 255;
    println!("Not yet overflowed: {}", overflow);

    // overflow = 257; /* Error overflowed*/
    // println!("Overflowed: {}", overflow);

    /* Float
    * Default float is f64
    */
    let float_1 = 1.5;
    let float_2: f32 = 2.5;
    println!("Floats: {} and {}", float_1, float_2);

    /*** Boolean ***/
    let success: bool = true;
    println!("Boolean: {}", success);

    /* Character
    * char use single quotes
    */
    let crab: char = '🦀';
    println!("Char: {}", crab);

    /* Tuple
    * Fixed size length
    * Elements inside it can have different type
    */
    let tuple = (100, 200.2, '🐧');
    println!("Tuple: {:?}", tuple);

    let tuple_2: (f64, char, &str) = (1.0, 'F', "What?");
    println!("Tuple: {:?}", tuple_2);

    // Destructor
    let (x, y, z) = tuple;
    println!("Destructor: {}, {}, {}", x, y, z);

    // Dot operator
    let x = tuple_2.2;
    println!("Dot operator: {}", x);

    /* Array
    * Fixed length
    * All elements have the same type
    * Data saved inside stack
    */
    let array = [1, 2, 3];
    println!("Array: {:?}", array);

    let array_2: [f64; 2] = [1.2, 2.1];
    println!("Array: {:?}", array_2);

    let array_3: [i32; 3] = [13;3];
    println!("Array: {:?}", array_3);

    

}