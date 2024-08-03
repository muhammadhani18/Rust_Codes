use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil}


struct myBox<T>(T);

impl<T> myBox<T> {
    fn new(x: T) -> myBox<T> {
        myBox(x)
    }
}

impl<T> Deref for myBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data:String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Droping CustomPointer with data {}", self.data);
    }
}
fn main() {
    let b: Box<i32> = Box::new(5);

    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    //////////////////////////////////////////////////////////
       
    let x: i32 = 5;
    let y: myBox<i32> = myBox::new(x);

    assert_eq!(5,x);
    assert_eq!(5, *y);

    let m = myBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer{
        data: String = String::from("other stuff");
    }
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}

fn hello(name: &str) {
    println!("Hello {}!", name);
}
