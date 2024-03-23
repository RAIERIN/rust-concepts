fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /*
    - Shadowing
    1. Binds to y to a value of 5
    2. Creates a new variable y
    3. Takes the original value and adding 1 so the value is 6
    4. in inner scope let statement also shadows y and creates new variable multiplying by 2 to give 12
    5. When the inner scope is over, the inner shadowing ends and y returns to being 6
    Output:
    The value of x in the inner scope is: 12
    The value of x is: 6
    */

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is:{y}");
    }

    println!("The value of y is: {y}");

    /*
     * Shadowing Example:
     * Here shadowing spares us from having to come up with different names, such as spaces_str and spaces_num
     * instead we can reuse the simpler spaces name
     * We won't be able to use mut it will show compile error
     */
    let spaces = " ";
    let spaces = spaces.len();
}
