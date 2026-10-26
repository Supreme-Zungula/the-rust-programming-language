fn main() {
    let greeting = String::from("hello world");
    let first = first_word(&greeting);
    println!("The first word from the greeting is '{first}'");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
