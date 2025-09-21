#[allow(dead_code)]
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) {
        println!("area is {}", self.height * self.width);
    }
}

pub fn run() {
    let user1 = User {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let mut user2 = User {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        sign_in_count: 1,
        active: false,
    };
    user2.email = String::from("bob@sample.com");
    println!("{:#?}", user1);
    println!("{:#?}", user2);

    let user3 = build_user(String::from("Charlie"), String::from("charlie@example.com"));
    println!("{:#?}", user3);

    let rect = Rectangle::create(30, 50);
    rect.area();
    println!("Rectangle: {:#?}", rect);
}

fn build_user(name: String, email: String) -> User {
    User {
        name,
        email,
        sign_in_count: 1,
        active: true,
    }
}
