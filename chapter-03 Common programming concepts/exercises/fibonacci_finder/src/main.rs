use std::io;

fn main() {
    println!("Enter a number you want to find a  Fibonaccie sequence for:");

    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let number: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let fibonacci = fibonacci_finder(number);
        println!("Fibonacci({number}) = {fibonacci}");
    }
}
fn fibonacci_finder(input_value: i32) -> i32 {
    if input_value == 1 {
        return input_value;
    }

    if input_value == 2 {
        return input_value;
    }

    let mut num1: i32 = 1;
    let mut num2: i32 = 2;
    let mut fibo: i32 = 1;
    let mut index = 3;

    while index <= input_value {
        fibo = num1 + num2;
        num1 = num2;
        num2 = fibo;
        index += 1;
    }

    fibo
}
