use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn communication () {
    let (tx,rx) = mpsc::channel();
    let (tx2,rx2) = mpsc::channel();

    thread::spawn( move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for v in vals {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn( move || {
        let vals = vec![
            String::from("hi2"),
            String::from("from2"),
            String::from("the2"),
            String::from("thread2"),
        ];
        for v in vals {
            tx2.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx {
        println!("Got {}", recieved);
    }

    for recieved in rx2 {
        println!("Got {}", recieved);
    }
}