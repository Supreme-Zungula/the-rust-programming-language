struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum GreetMessage {
    Hello { id: i32 },
}

fn main() {
    // Matching multiple patterns.
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching range of values with ..=
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("Something else"),
    }

    let x = 'c';
    match x {
        'c'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructing to break apart values.
    // ----------------------------------

    // Structs
    println!("\nMatching Structs");
    let p = Point { x: 0, y: 7 };

    // variables in the pattern don't have match fields in the struct.
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("One neither axis: ({x}, {y})"),
    }

    // Enums
    // ----------------------------------------
    println!("\nMatch enums");
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g} and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s} and value {v}");
        }
    }

    // Adding Conditionals with Match guards
    // ---------------------------------------------
    println!("\nAdding Conditionals with Match Guards");
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x =  {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    // Using @ Bindings
    // ------------------------------------------

    let msg = GreetMessage::Hello { id: 5 };

    match msg {
        GreetMessage::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}")
        }
        GreetMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        GreetMessage::Hello { id } => println!("Found some other id: {id}"),
    }
}
