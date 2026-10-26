use std::collections::HashMap;

fn main() {
    // Creating a new hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing values in a  hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name} team score is {score}");

    // Iterating over a hash map.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Adding a Key and Value only if a key isn't present.
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("{scores:?}");

    // Updating a Value based on the old value.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}
