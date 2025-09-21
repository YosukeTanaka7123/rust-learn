pub fn run() {
    let rs1 = division_option(5.0, 2.0);
    match rs1 {
        Some(x) => println!("Result: {:.3}", x),
        None => println!("Division by zero"),
    }

    let rs2 = division_option(5.0, 0.0);
    match rs2 {
        Some(x) => println!("Result: {:.3}", x),
        None => println!("Division by zero"),
    }

    let rs3 = division_result(5.0, 2.0);
    match rs3 {
        Ok(x) => println!("Result: {:.3}", x),
        Err(e) => println!("Error: {}", e),
    }

    let rs4 = division_result(5.0, 0.0);
    match rs4 {
        Ok(x) => println!("Result: {:.3}", x),
        Err(e) => println!("Error: {}", e),
    }

    let ar5 = [1, 2, 3];
    let rs5 = sum(&ar5);
    match rs5 {
        Some(x) => println!("Sum: {}", x),
        None => println!("Not enough elements"),
    }

    let ar6 = [1, 2];
    let rs6 = sum(&ar6);
    match rs6 {
        Some(x) => println!("Sum: {}", x),
        None => println!("Not enough elements"),
    }
}

fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 { None } else { Some(x / y) }
}

fn division_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Division by zero!"))
    } else {
        Ok(x / y)
    }
}

fn sum(a: &[i32]) -> Option<i32> {
    let a0 = a.get(0)?;
    let a1 = a.get(1)?;
    let a2 = a.get(2)?;
    Some(a0 + a1 + a2)
}
