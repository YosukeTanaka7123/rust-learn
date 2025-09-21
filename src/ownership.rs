pub fn run() {
    let s1 = String::from("hello");
    let _s2 = s1;

    // println!("{}, world!", s1); // This line would cause a compile-time error because s1 has been moved to s2

    let i1 = 1;
    let i2 = i1;
    println!("i1: {}, i2: {}", i1, i2); // This works because integers implement the Copy trait
    println!("Stack address of i1: {:p}", &i1);
    println!("Stack address of i2: {:p}", &i2);

    let sl1 = "literal";
    let sl2 = sl1;
    println!("sl1: {}, sl2: {}", sl1, sl2); // This works because string literals are stored in the binary's data segment and have a 'static lifetime
    println!("Stack address of sl1: {:p}", &sl1);
    println!("Stack address of sl2: {:p}", &sl2);

    let s3 = String::from("hello");
    let s4 = s3.clone(); // Deep copy of the heap data
    println!("s3: {}, s4: {}", s3, s4); // This works because s3 is cloned
    println!("Stack address of s3: {:p}", &s3);
    println!("Stack address of s4: {:p}", &s4);
    println!("Heap address of s3 data: {:p}", s3.as_ptr());
    println!("Heap address of s4 data: {:p}", s4.as_ptr());

    let s5 = String::from("hello");
    println!("s5 before function call: {}", s5);
    println!("Stack address of s5: {:p}", &s5);
    println!(
        "Heap address of s5 data: ptr: {:p}, len: {}, cap: {}",
        s5.as_ptr(),
        s5.len(),
        s5.capacity()
    );
    takes_ownership(s5); // s5 is moved into the function
    // println!("{}", s5); // This line would cause a compile-time error because s5

    let s6 = String::from("hello");
    println!("s6 before function call: {}", s6);
    println!("Stack address of s6: {:p}", &s6);
    println!(
        "Heap address of s6 data: ptr: {:p}, len: {}, cap: {}",
        s6.as_ptr(),
        s6.len(),
        s6.capacity()
    );
    let s7 = take_give_back(s6); // s6 is moved into the function and returned
    // println!("{}", s6); // This line would cause a compile-time error because s6 has been moved
    println!("s7 after function call: {}", s7);
    println!("Stack address of s7: {:p}", &s7);
    println!(
        "Heap address of s7 data: ptr: {:p}, len: {}, cap: {}",
        s7.as_ptr(),
        s7.len(),
        s7.capacity()
    );

    let s8 = String::from("hello");
    let s8_len = calculate_length(&s8); // Pass a reference to s8
    println!("The length of '{}' is {}.", s8, s8_len);

    let mut s9 = String::from("hello");
    println!("s9 before change: {}", s9);
    change(&mut s9);
    println!("s9 after change: {}", s9);

    let s10 = String::from("hello");
    let r1 = &s10; // No problem
    let r2 = &s10; // No problem
    // let r3 = &mut s10; // BIG PROBLEM
    println!("s10: {}, r1: {}, r2: {}", s10, r1, r2);

    let mut s11 = String::from("hello");
    let r1 = &mut s11; // No problem
    // println!("s11: {}", s11); // BIG PROBLEM
    println!("r1: {}", r1);
    println!("s11: {}", s11);

    // Dangling reference example (commented out to avoid compile-time error)
    // let s12;
    // {
    //     let ss1 = 10;
    //     // s12 = &ss1; // This line would cause a compile-time error because ss1 does not live long enough
    // }
    // println!("s12: {}", *s12);
}

fn takes_ownership(s: String) {
    println!("takes_ownership param is {}", s);
    println!("Stack address of param: {:p}", &s);
    println!(
        "Heap address of param data: ptr: {:p}, len: {}, cap: {}",
        s.as_ptr(),
        s.len(),
        s.capacity()
    );
}

fn take_give_back(s: String) -> String {
    println!("take_give_back param is {}", s);
    println!("Stack address of param: {:p}", &s);
    println!(
        "Heap address of param data: ptr: {:p}, len: {}, cap: {}",
        s.as_ptr(),
        s.len(),
        s.capacity()
    );
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
