fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // pass s1 by reference.

    println!("The lenght of '{s1}' is {len}");

    let mut s = String::from("hello");
    change_str(&mut s);
    println!("The value of 's' is '{s}' after mutation.");
}

// Note: s is immutable by default.
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_str(some_string: &mut String) {
    some_string.push_str(", world");
}
