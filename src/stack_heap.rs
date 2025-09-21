enum _List {
    Node(i32, Box<_List>),
    Nil,
}

pub fn run() {
    let _a1: [u8; 7_000_000] = [1; 7_000_000];
    // let _a1: [u8; 9_000_000] = [1; 9_000_000];

    let mut v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![6, 7];
    let mut v3 = vec![8, 9];
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);

    println!("Stack address of v1: {:p}", &v1);
    println!("Stack address of v2: {:p}", &v2);
    println!("Stack address of v2: {:p}", &v3);

    println!(
        "Heap memory of v1 ptr: {:p}, len: {}, cap: {}",
        v1.as_ptr(),
        v1.len(),
        v1.capacity()
    );
    println!(
        "Heap memory of v2 ptr: {:p}, len: {}, cap: {}",
        v2.as_ptr(),
        v2.len(),
        v2.capacity()
    );
    println!(
        "Heap memory of v3 ptr: {:p}, len: {}, cap: {}",
        v3.as_ptr(),
        v3.len(),
        v3.capacity()
    );
    v1.insert(1, 10);
    v1.remove(4);
    v1.append(&mut v3);
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);

    // box pointer example
    let t1 = (10, String::from("hello"));
    println!("Stack address of t1: {:p}", &t1);
    println!(
        "Heap memory of t1.1 ptr: {:p}, len: {}, cap: {}",
        t1.1.as_ptr(),
        t1.1.len(),
        t1.1.capacity()
    );

    let mut b1 = Box::new(t1);
    (*b1).1 += " world!";
    println!("b1: {:?}", b1);
    println!("Stack address of b1: {:p}", &b1);
    println!("Heap address of b1: {:p}", b1);
}
