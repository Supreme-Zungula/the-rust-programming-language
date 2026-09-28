fn main() {
    let mut s = String::from("Hello"); // s is valid from th is point forward.
    s.push_str(", world!");
    println!("{s}");
} // this scope ins now over, an s is no longer valid.
