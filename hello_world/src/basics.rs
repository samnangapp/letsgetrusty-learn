pub fn variables() {
    println!("--- Variables and Mutability ---");

    // Immutable variable
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // This would cause a compile-time error

    // Mutable variable
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The constant value is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }
    println!("The value of z is: {z}");
}

pub fn data_types() {
    println!("\n--- Data Types ---");
    // Scalar Types
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'R';

    println!("Integer: {integer}, Float: {float}, Boolean: {boolean}, Character: {character}");

    // Compound Types
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Tuple values: {x}, {y}, {z}");

    // Array
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    println!("First month: {}", months[0]);
}

pub fn functions() {
    println!("\n--- Functions ---");
    another_function(5);
    let x = plus_one(5);
    println!("The value of x (5 + 1) is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // Expression, returns value
}

pub fn control_flow() {
    println!("\n--- Control Flow ---");
    // if expression
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result of the loop is {result}");

    // while
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}
