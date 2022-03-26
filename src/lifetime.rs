#![allow(dead_code)]

/* Lifetime
* Scope area whcich is reference is still valid
* there are some purpose of using the lifetime in Rust
*/

pub fn lifetimes() {
    /* [1] Prevent  Dangling References */
    // bellow code will give errors
    {
        let err;
        {
            let x = 12; // x only has lifetime until end of this scope
            err = &x; // but we try to pass reference of x into err
        }

        // println!("err: {}", err); // and we call the err which is has a undefined value (in Rust we don't have NULL)
    }

    let long = String::from("Outside");
    {
        let short = String::from("Inside");
        let result = longest(long.as_str(), short.as_str());
        println!("The lifetime: {}", result);
    }
    // println!("The lifetime: {}", result); // ERROR
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
