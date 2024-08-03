use std::thread;

fn simple_threads() {
    let v = vec![1,2,3];

    let handle = thread::spawn( move || {
    println!("here's a vector: {:?}", v);
    });
    handle.join().unwrap(); //wait for all threads to complete
}
