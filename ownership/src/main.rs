/**
 * Each value in rust has an owner
 * There can only be one owner at a time
 * When the owner goes out of scope, the value will be dropped
 */

fn main() {
    {
        // s is not valid here, it's not yet declared
        let s = String::from("hello"); // s is valid from this point forward
                                       // do stuff with s
        println!("{}", s);
    } // this scope is now over, and s is no longer valid

    let mut s = String::from("hello");
    s.push_str(", world!"); // appends a literals to a String
    println!("{}", s);

    /*
     *Double free error
     */

    /*
     * Notes: Rust will never automatically create "deep" copies of your data, Therefore, any automatic copying can be assumed be be inexpensive in terms of runtime performance
     */
    let s1 = String::from("hello");
    let s2 = s1; // Here s1 is moved into s2. So s1 is removed and is reassigned to s2
    println!("{}", s2);

    /*
     * If you want to deeply copy the heap data of the String, we can use method called clone.
     */

    let a1 = String::from("value");
    let a2 = a1.clone();

    println!("s1 = {}, s2={}", a1, a2);

    let a = String::from("hello"); // a comes into scope
    takes_ownership(a); // a's value moves into the function and so is no longer valid here

    let b = 5; // x comes into scope

    makes_copy(b); // x would move into the function
                   // but i32 is Copy, so its okay to still use x afterward
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. the backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens

fn return_variables() {
    let s1 = gives_ownership_return(); // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership_return() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
