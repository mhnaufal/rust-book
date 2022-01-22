#![allow(dead_code)]

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    status: bool,
}

/* Tuple Struct */
struct Color(u32, u32, u32);

/* Unit-like struct */
struct Empty();

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /* Constructor or Associated Function*/
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    /* Area method */
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

}

pub fn r#struct() {
    let mut user_1 = User {
        username: String::from("MRX"),
        email: String::from("mrx@secmail.en"),
        status: true,
    };

    user_1.status = false;

    let _black = Color(0, 0, 0);

    let mut rect_1 = Rectangle::square(2);
    println!("Create rect: {:#?}", rect_1);
    println!("Get the area of rect_1: {}", rect_1.area());
    println!("Get the width of rect_1: {}", rect_1.get_width());
    rect_1.set_width(5);
    println!("Get the width of rect_1 after: {}", rect_1.get_width());
}