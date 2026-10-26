use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // If not found, try to create the file.
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Prbolem creating the file: {e:?}"),
            },
            _ => {
                panic!("Prbolem opening the file: {error:?}");
            }
        },
    };
    println!("Greeting file is: {greeting_file:?}");

    // Read user name from file.
    let username = read_username_from_file2().expect("Could not read useraname from file");
    println!("username is: {username:?}");
}

// Error propagation
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Implementing read_username_from_file using the ? operator.
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;

    Ok(username)
}
