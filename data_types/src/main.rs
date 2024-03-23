use std::io;
/*
* Every value in Rust is certain data type.
* Two data type: Scalar and Compound
* NOTE: Rust is statically typed language i.e. should know the type at compile time
*/

/*
* Types:
* 1. Scalar type: single value i.e. integers, floating-point, numbers, Booleans and characters
* 2. Compound: tuples and arrays
 * -tuples have fixed length, once declared they cannot grow or shrink in size
 * - Array has a fixed length
*/

fn main() {
    // addition
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    let reminder = 43 % 4;

    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructure tuple
    let (x, y, z) = tup;

    let five_hundred = tup.0;

    // Array Type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Same value of 3 with length 5
    let a = [3; 5];

    // Accessing array
    let first = a[0];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");
}
