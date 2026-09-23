fn main() {
    println!("Hello, world!");

    another_function(5);
    let x = plus_one(6);
    println!("The value of x + 1 is: {}", x);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// Functions with Return values

fn plus_one(x: i32) -> i32 {
    x + 1
}
