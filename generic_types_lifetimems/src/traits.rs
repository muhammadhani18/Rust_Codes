//A trait defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way.
//Traits are similar to a feature often called interfaces in other languages, although with some differences.

pub trait Summary{
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

imp Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self,location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
//traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}


// we can also specify more than one trait bound
pub fn notify(iten: &(impl Summary + Display)) {
    
}


//functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read. 

fn some_function<T, U>(t: &T, u: &U) -> i32 
where 
    T: Display + Clone,
    U: Clone + Debug,
{
    
}

//returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Ofcourse, as you probably know"),
        reply: false,
        retweet: false,
    }
}




