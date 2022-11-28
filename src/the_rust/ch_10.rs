use std::fmt::{Debug, Display};

// generics
pub fn generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    let result_ref: char;
    println!("The largest number is {}", result);
    {
        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);

        println!("The largest char is {}", result);
        result_ref = *result;
    }
    println!("The largest char is {}", result_ref);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list.iter() {
        if number > largest {
            largest = number;
        }
    }

    &largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Trait
pub fn ch_10_trait() {
    let summarized = returns_summarizable();
}
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
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
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//糖衣構文
// pub fn notify(item :&impl Summary){
//     println!("Breaking News! {}", item.summarize());
// }

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking News! {}", item.summarize());
}

pub fn notify2<T: Summary + Display>(item: &T) {}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

fn returns_summarizable() -> impl Summary {
    //1種類の型を返すときにimpl Traitを使える
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

//lifetime
pub fn ch_10_lifetime() {
    let mut r = String::from("first");
    {
        let x: &'static str = "static str";
        r = x.to_string();
    }
    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    // 最長の文字列は、{}です
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    //"'.'が見つかりませんでした"
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
