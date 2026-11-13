use std::thread;
use std::time::Duration;

fn main() {
    // Creating a new thread with spawn.
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} form the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    // Waiting for the thread to finish.
    handle.join().unwrap();

    // using move Closures with threads.
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
