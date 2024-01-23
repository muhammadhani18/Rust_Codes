//generics are used to effectively handle duplications of concepts. Functions can take parameters of some generic types. 
// lifetimes: a variety of generics that give the compiler information about how references relate to each other. Lifetimes allow us to give the compiler enough information about borrowed values so that it can ensure references will be valid in more situations than it could without our help.

//generics allow code to operate on abstract types
//We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.

use aggregate::{Summary, Tweet};

fn largest<T>(list : &[T]) -> &T {
    let mut largest = &list[0];
    
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
} 

fn main() {
    let number_list = vec![34,50,25,100,65];

    let result = largest(&number_list);

    println!("The largest number is: {}", result);
    
    //---------------------------------------

    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know what you're doing"),
        reply: false,
        retweet: false,
    };   

    println!("1 new tweet: {}",tweet.summarize());
}
