use std::fmt::format;

pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {} ...", self.summarize_author())
    }

}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     // fn summarize(&self) -> String {
//     //     format!("{}, by {} ({})", self.headline, self.author, self.location)
//     // }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

pub fn return_summarizeble() -> impl Summary {
    Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    }
}