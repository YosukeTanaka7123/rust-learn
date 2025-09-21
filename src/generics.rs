struct Point<T> {
    _x: T,
    _y: T,
}

struct Point2<T, U> {
    _x: T,
    _y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            _x: self._x,
            _y: other._y,
        }
    }
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest_manual = number_list[0];
    for &number in &number_list {
        if number > largest_manual {
            largest_manual = number;
        }
    }
    println!("The largest_manual is {}", largest_manual);
    println!("The largest func result is {}", largest_i32(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest(&char_list));
    println!("The largest number is {}", largest(&number_list));

    let _p1 = Point { _x: 5, _y: 10 };
    // let _p2 = Point { _x: 1.0, _y: 4 };
    let _p2 = Point2 { _x: 1.0, _y: 4 };
    let p3 = Point2 { _x: 'H', _y: 10.4 };
    let p4 = p3.mixup(Point2 { _x: 5, _y: "World" });
    println!("p4.x = {}, p4.y = {}", p4._x, p4._y);
}

fn largest_i32(list: &Vec<i32>) -> i32 {
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
