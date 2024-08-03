use std::sync::{Arc, Mutex};
use std::thread;

fn shared_state () {
    let count = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let count = Arc::clone(&count);

        let handle = thread::spawn(move || {
            let mut num = count.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *count.lock().unwrap());
}