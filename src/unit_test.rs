#![allow(dead_code)]

struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn compare_area(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
}

fn double_value(a: i32) -> i32 {
    a * 2
}

fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_is_larger() {
        let a = Rectangle {
            width: 10,
            height: 20,
        };
        let b = Rectangle {
            width: 5,
            height: 15,
        };
        assert!(a.compare_area(&b));
    }

    #[test]
    fn test_a_is_smaller() {
        let a = Rectangle {
            width: 5,
            height: 10,
        };
        let b = Rectangle {
            width: 10,
            height: 20,
        };
        assert!(!a.compare_area(&b));
    }

    #[test]
    fn test_double_value() {
        assert_eq!(8, double_value(4));
        assert_eq!(-6, double_value(-3));
    }

    #[test]
    fn test_greeting() {
        assert_eq!("Hello, Alice!", greeting("Alice"));
        assert_eq!("Hello, Bob!", greeting("Bob"));
    }
}
