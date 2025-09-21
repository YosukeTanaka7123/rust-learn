// pub mod sub_a;
// pub mod sub_b;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars module!");
    // sub_a::func_a();
    // sub_b::func_b();

    let mut x: i32 = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    let _y: i32 = 3;
    let _f: f64 = 0.2;

    println!("MAX_POINTS = {}", MAX_POINTS);

    println!("{}", usize::BITS);
    println!("MAX_POINTS address = {:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i32 = 2;
    let i4: i32 = 3;
    let i5: i64 = 1;
    println!("Stack address: {:p}", &i2);
    println!("Stack address: {:p}", &i3);
    println!("Stack address: {:p}", &i4);
    println!("Stack address: {:p}", &i5);

    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("The value of y is: {}, {}, {}", t1.0, t1.1, t1.2);

    let _a1 = [1, 2, 3, 4, 5];
    let _a2 = [0; 10];

    let s1 = "hello ワールド";
    let s2 = "hello world";
    println!("Stack address of s1: {:p}", &s1);
    println!("Static memory address of s1: {:p}", s1.as_ptr());
    println!("Static memory bytes of s1: {}", s1.len());
    println!("Stack address of s2: {:p}", &s2);
    println!("Static memory address of s2: {:p}", s2.as_ptr());
    println!("Static memory bytes of s2: {}", s2.len());

    let mut str1 = String::from("hello");
    let mut str2 = String::from("hello world");
    println!("Stack address of str1: {:p}", &str1);
    println!("Heap memory address of str1: {:p}", str1.as_ptr());
    println!(
        "Heap memory bytes of str1: {}, len = {}, capacity = {}",
        str1,
        str1.len(),
        str1.capacity()
    );
    println!("Stack address of str2: {:p}", &str2);
    println!("Heap memory address of str2: {:p}", str2.as_ptr());
    println!(
        "Heap memory bytes of str2: {}, len = {}, capacity = {}",
        str2,
        str2.len(),
        str2.capacity()
    );
    str1.push_str(" world");
    str2.push_str("!!!");
    println!("After push: {}", str1);
    println!("After push: {}", str2);
}
