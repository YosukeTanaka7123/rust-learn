pub fn run() {
    let st1 = String::from("X");
    let st2 = String::from("y");
    let rs1: &str = get_longest(&st1, &st2);
    println!("st1 is {}", st1);
    println!("st2 is {}", st2);
    println!("rs1 is {}", rs1);

    let st3 = String::from("x");
    let rs2;
    {
        let st4 = String::from("y");
        rs2 = get_longest(&st3, &st4);
        println!("st3 is {}", st3);
        println!("st4 is {}", st4);
        println!("rs2 is {}", rs2);
    }
    // println!("rs2 is {}", rs2); // This line will cause a compile-time error because st4 does not live long enough
}

fn get_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn _dummy() -> String {
    let s = String::from("hello");
    s
}

// fn dummy1<'a>() -> &'a str {
//     let s = String::from("hello");
//     &s // This line will cause a compile-time error because s does not live long enough
// }

// fn dummy2<'a>() -> &'a i32 {
//     let x = 10;
//     &x // This line will cause a compile-time error because x does not live long enough
// }
