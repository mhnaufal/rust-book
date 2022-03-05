#![allow(dead_code)]

/* Trait
* scope trait trait_name
* there is no implementation body in the trait
* only declaration
*/
pub trait Summary {
    fn summarize(&self) -> String;
    fn check_author(&self) -> String {
        String::from("Anonymous")
    }
}

pub struct Article {
    pub headline: String,
    pub content: String,
    pub author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!(
            "📌 {} by ({})\n{}",
            self.headline, self.author, self.content
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} 🐣 Tweet [{}]", self.username, self.content)
    }
}

pub fn traits() {
    let my_tweet = Tweet {
        username: String::from("ikiuyuu"),
        content: String::from(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod",
        ),
        reply: false,
    };

    println!("There is a new tweet!: {}", my_tweet.summarize());
    println!("🔼 tweet by {}", my_tweet.check_author());
}
