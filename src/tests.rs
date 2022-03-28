#[cfg(test)]
mod test {
    #[test]
    pub fn tests() {
        assert_eq!(1 + 2, 3);
    }

    #[test]
    fn failed() {
        assert_ne!(2 + 1, 4);
    }
}

/* Assert macro
* if an argument evaluate to true, the test passes, else it throws panic!
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn is_can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 9,
        height: 10,
    };

    let smaller = Rectangle {
        width: 6,
        height: 4,
    };

    assert!(larger.is_can_hold(&smaller));
}

fn greeting(name: &str) -> String {
    format!("Good morning")
}

#[test]
fn greeting_test() {
    let result = greeting("Batman");

    assert!(
        result.contains("Batman"),
        "`greeting` function should contain name, value was `{}`",
        result
    );
}
