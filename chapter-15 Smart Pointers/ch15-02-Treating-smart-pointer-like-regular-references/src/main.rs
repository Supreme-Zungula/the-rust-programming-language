use std::ops::Deref;

// Defining our own smart pointer.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Using  Box<T> like a reference.
    //--------------------------------
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    //Defining our own smart pointer.
    let x = 6;
    let y = MyBox::new(x);

    assert_eq!(6, x);
    assert_eq!(6, *y);

    // Using deref coreciion in functions and methods.
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
