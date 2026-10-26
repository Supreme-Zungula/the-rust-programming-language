fn main() {
    let mut s = String::from("foo");
    // Appending string with push_str or push.
    s.push_str("bar");
    println!("s is '{s}'");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is '{s2}'");

    // push takes a single character.
    let mut another_str = String::from("lo");
    another_str.push('l');
    println!("another_str is '{another_str}'");

    // Concatenation with + and format.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Not s1 has been move here and can no longer be used.
    println!("s2 is {s2}");
    println!("s3 is {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s is '{s}'");

    // Slicing strings

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("&hell[0..4] is {s}");

    //Interating over strings
    for c in s.chars() {
        println!("{c}");
    }
}
