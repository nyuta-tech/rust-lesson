trait Fruits {
    fn price(&self) -> u32;
}

struct Apple;

impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}

struct Banana;

impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

trait Summary {
    //デフォルトの関数になる
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} ,by {}({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

impl Message for Tweet {
    fn message(&self) -> String {
        format!("{}", self.content)
    }
}

trait Message {
    fn message(&self) -> String {
        String::from("Message")
    }
}
pub fn run() {
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);

    let tweet = Tweet {
        username: String::from("nyuta"),
        content: String::from("of cource, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet {}", tweet.summarize());

    notify_another(&tweet);
}

fn get_price<T: Fruits>(fruits: T) {
    println!("price is {}", fruits.price());
}

fn notify(item: &impl Summary) {
    println!("news! {}", item.summarize());
}

fn notify_another(item: &(impl Summary + Message)) {
    println!("news! {}", item.summarize());
    println!("message! {}", item.message());
}
