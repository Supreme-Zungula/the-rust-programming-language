fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There's no third element."),
    }

    // Iterating over a vector
    for i in &v {
        println!("{i}");
    }

    // iterating over mutable references to each element in a mutable vector
    let mut another_vect = vec![1, 2, 3, 4];
    for i in &mut another_vect {
        *i *= 50;
        println!("i * 50 = {i}");
    }

    // using an Enum to store mutiplpe types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(20.10),
    ];
}
