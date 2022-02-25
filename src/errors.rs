#![allow(dead_code)]

use std::{
    fs::{self, File},
    io::ErrorKind,
    panic,
};

/// 2 types of Error in Rust
/// Recoverable & Unrecoverable
/// Recoverable = User error e.g. missing file -- Result<T, E>>
/// Unrecoveralbe = System error e.g. array out of bound -- panic!

pub fn errors() {
    /* Unrecoverable Error with panic! */

    // panic!("Manual panic error");

    // RUST_BACKTRACE=1 cargo run
    // to get 'debug' like data from panic

    /* Recoverable Error with Result enum */
    let f = File::open("src/mom.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Error while opening the file: {:?}", error),
    };

    println!("File = {:?}", f);

    let f = File::open("src/mom.txt");

    let g = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("src/mom.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem while creating a file: {:?}", e),
            },
            other_error => {
                panic!("Problem while opening the file: {:?}", other_error)
            }
        },
    };

    println!("File 2 = {:?}", g);

    /* unwrap & expect */
    let h = File::open("src/mom.txt").unwrap();
    let _h = File::open("src/mom.txt").expect("Error 3 while opening the mom.txt");

    println!("File 3 = {:?}", h);

    let i = fs::read_to_string("src/mom.txt").expect("Something error while reading the file");

    println!("File 4 = {:?}", i);
}
