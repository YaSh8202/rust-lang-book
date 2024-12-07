use std::{
    fmt::{Debug, Display},
    iter::Sum,
};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{} by {}", self.headline, self.author)
    // }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

// pub fn notify(item: &impl Summary){
//     println!("Breaking news! {}", item.summarize());
// }
// pub fn notify<T: Summary>(item: &T){
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary, U: Summary>(item1: &T, item2: &U) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    5
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

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Horse"),
        content: String::from("I am a horse"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("Horse"),
        content: String::from("I am a horse"),
        reply: false,
        retweet: false,
    };

    // println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("Breaking News"),
        content: String::from("This is a breaking news"),
    };

    // println!("New article available! {}", article.summarize());

    notify(&tweet, &article);


    println!("{}",returns_summarizable().summarize());
    
}
