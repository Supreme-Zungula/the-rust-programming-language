fn main() {
    let s = String::from("hell");

    // s's value movers into the  functions..
    // and so is no longer valid after here.
    takes_ownership(s);

    let x = 5;

    // Because i32 implements the copy trait
    // x does NOT move into the function. So, it's okay to use x afterward.
    makes_copy(x);

    let str = String::from("Test");
    let (s1, len) = calculate_length(str);
    println!("The length of '{s1}' is {len}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_int: i32) {
    println!("{some_int}");
}

// Returns multiple values using a tuple.
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
