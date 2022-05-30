use std::fmt::Display;

fn main() {
    let n = Tweet {
        title: String::from("wow"),
    };
    println!("{}", news::<Tweet>(n));
}

pub trait Summary {
    fn summarize(&self) -> String;
}

// ------------------------------

pub struct NewsArticle {
    title: String,
    subTitle: String,
    body: Vec<String>,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("news: {} {} ...", self.title, self.subTitle)
    }
}

// ------------------------------

#[derive(Clone)]
pub struct Tweet {
    title: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("tweat: {} ...", self.title)
    }
}

// ------------------------------

pub trait Keywords: Summary + Clone {
    fn keywords(&self) -> String;
}

impl Keywords for Tweet {
    fn keywords(&self) -> String {
        String::from("extracted keywords: ...")
    }
}

// ------------------------------

pub fn news<T: Summary>(n: T) -> String {
    n.summarize()
}

// -----------------------------------------------

pub enum Enum1 {
    A(String),
    B(i32),
    C(Tweet),
}

pub enum Option<T> {
    None,
    Some(T),
}

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
