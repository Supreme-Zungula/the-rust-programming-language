use std::io;

fn main() {
    println!("Enter temperature to convert to from degrees Fahrenheit to Celsius");
    loop {
        let mut input_value = String::new();

        io::stdin()
            .read_line(&mut input_value)
            .expect("Failed to read line");

        let fahrenheit_temp: i32 = match input_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let celsius_temp: i32 = convert_fahrenheit_celsius(fahrenheit_temp);
        println!("{fahrenheit_temp} fahrenheit = {celsius_temp} celsius");
    }
}

fn convert_fahrenheit_celsius(fahrenheit: i32) -> i32 {
    let celsius = fahrenheit - 32 * 5 / 9;

    return celsius;
}
