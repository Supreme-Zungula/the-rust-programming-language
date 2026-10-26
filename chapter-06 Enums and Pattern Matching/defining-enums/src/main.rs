use std::option;

enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // Standard libary option enum
    let x: i8 = 6;
    let y: Option<i8> = Some(5); // can be i8 value on None

    // Cannot add definite value x to optional value y.
    let sum = x + y;
}
