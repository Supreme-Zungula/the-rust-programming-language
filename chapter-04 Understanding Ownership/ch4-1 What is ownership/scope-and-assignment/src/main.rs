fn main() {
    let mut s = String::from("hello");
    s = String::from("ahoy"); // the value 'hello' is dropped from the here since s is assigned a
    // new value

    println!("{s}, hello"); // prints "ahoy, world"
}
