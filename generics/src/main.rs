use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Summary2 {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['a', 'x', 'c', 'y', 'd', 'r'];
    let integer = Point { x: 5, y: 5 };
    let float = Point { x: 7.3, y: 5.4 };
    println!("The largest char is {}", largest(&char_list));
    println!("The largest number is {}", largest(&number_list));
    println!("Integer Point x: {}", integer.x());
    println!("Float Point x: {}", float.x());
    println!("Distance from origin: {}", float.distance_from_origin());

    // Lifetimes. Heavy stuff..
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {result}");
    }

    // let string1 = String::from("abcd");
    // let result;
    // {
    //     let string2 = String::from("xyzddf");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");
}

fn longest2<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub struct NewsArticle2 {}
impl Summary2 for NewsArticle2 {
    fn summarize_author(&self) -> String {
        String::from("None")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
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

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
