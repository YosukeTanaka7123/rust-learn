trait Fruits {
    fn price(&self) -> u32;
}

struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        3
    }
}

struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        2
    }
}

trait Summary {
    fn summarize(&self) -> String;
}
trait Message {
    fn message(&self) -> String {
        String::from("(Read more...)")
    }
}

#[allow(dead_code)]
struct NewsArticle {
    headline: String,
    location: String,
    auther: String,
    content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.auther, self.location)
    }
}
impl Message for NewsArticle {
    fn message(&self) -> String {
        format!("Read more from {}...", self.auther)
    }
}

#[allow(dead_code)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

pub fn run() {
    let apple = Apple;
    let banana = Banana;

    println!("Apple price: {}", get_price(&apple));
    println!("Banana price: {}", get_price(&banana));
    println!("{}", apple.price());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        auther: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
    notify(&article);
    // notify(&tweet); // Error: the trait `Message` is not implemented for `Tweet`
}

fn get_price<T: Fruits>(item: &T) -> u32 {
    item.price()
}

fn notify(item: &(impl Summary + Message)) {
    println!("Message comming! {}", item.message());
    println!("Breaking news! {}", item.summarize());
}
