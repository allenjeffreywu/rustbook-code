// you can run this file with `cargo run --bin traits`

// trait defines the functionality a particular type has and can share with other types
// we can use trait bounds to specify that a generic type can be any type that has certain behavior (similar to interfaces)

// Defining a trait
pub trait Summary {
    // no implementation here, let those that have this trait implement it instead

    // it's also possible to have default implementations use other default implementations
    fn summarize_author(&self) -> String;

    // if you want a default implementation of a method you can define it here.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
// trait keyword
// pub so that crates depending on this crate can make use of this trait too

// Implementing a trait on a type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
// similar to implementing regular methods
// difference is that after impl, we put the trait name we want to implement, then for keyword, then the name of the type

// not needed here because wew are not importing from aggregator! we didn't even make it
// use aggregator::{Summary, Tweet};
// you can bring summary trait into scope to implement summary on your own types
// Restriction: we can implement a trait on a type ONLY if either the trait or the type, or both, are local to our crate
// we cannot implement Display trait on Vec<T> here because they are both external
// orphan rule - the parent type must be present to implement.

// default implementations
// you can use a default implementation by doing:
// impl Summary for NewsArticle {}
// if empty, then we assume the default implementations for NewsArticle

// traits as parameters

fn main() {
    println!("Hello Allen");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
