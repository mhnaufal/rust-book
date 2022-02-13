#![allow(dead_code)]

use std::collections::HashMap;

pub fn map() {
  /* Create an empty HashMap */
  let mut scores = HashMap::new();
  println!("Hash map = {:?}", scores);

  /* Insert a key-value into the HashMap */
  scores.insert(String::from("Blue"), 13);
  scores.insert(String::from("Red"), 21);

  /* Get the value from HashMap */
  println!(
    "Blue {:#?}",
    scores
      .get(&String::from("Blue"))
      .ok_or("There is no Blue team")
  );

  /* Iterate through the HashMap */
  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }

  /* Updatning the HashMap */
  scores.insert(String::from("Blue"), 33);
 println!("New Blue {:#?}", scores.get(&String::from("Blue")).unwrap());

}
