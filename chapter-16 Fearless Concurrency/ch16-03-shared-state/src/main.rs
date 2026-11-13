use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // The API of Mutex<T>
    // --------------------------------
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {m:?}");

    // Shared access to Mutex<T>
    // ---------------------------------
    let counter = Arc::new(Mutex::new(0));
    let mut handles_list = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles_list.push(handle)
    }

    for handle in handles_list {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
