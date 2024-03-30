/*
* function uses snake case (function_name)
* Statements are instructions that perform some action and do not return a value
* Expressions evaluate to a resultant value
*/

fn main() {
    print_labeled_measurement(5, 'h');

    // If no semicolon you turn into the expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
