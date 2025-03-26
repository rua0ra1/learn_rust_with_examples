use core::num;
use std::process::ChildStdout;

fn main() {
    let x = 5;
    println!(" the value of the x is : {}", x);
    let x = "six";
    println!(" the value of the x is : {}", x);
    const SUBSCRIBER_COUNT: i32 = 100_000;
    // you can't set const return value
    println!(" the value of the sub is : {}", SUBSCRIBER_COUNT);

    // shadowing is creating the new variable using exisitng variable

    // scalar data type and compung data type
    // int float booleans characters (i32)

    // let a= 98_222; // Decimal
    // let b: =0xff; // hex
    // let c=0o77;// octal
    // let e: =b'A;

    // compund types
    let tup = ("lets get rust", 100_000);
    // get the values from tuple using two ways
    // 1) destructing
    let (_channel, _sub_count) = tup;
    // 2) dotting
    let _sub_count = tup.1;

    // arrays
    let error_codes = [200, 100, 500];
    let _not_found = error_codes[1];

    // intilaize the array with 0 and size 8
    let _my_array = [0; 8];

    println!("the return valus is {}", my_functions(1, 2));

    // in rust you can think of a piece of code as
    // either statement or expression
    // statemetn perform an action but no return a value, where as expression perfprms an action return an value

    // control flow

    // let number = 32;
    // if number < 10 {
    //     print!(" the first condition is met");
    // } else if number < 22 {
    //     print!(" the second condition is met");
    // } else {
    //     println!(" the second condition was false");
    // }
    // // inline if else statement
    // let condition = true;
    // let number = if condition { 5 } else { 6 };

    // three different kinds of loop in rust

    // 1 is loop
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    print!(" the counter is : {}", counter);

    // 2 is while

    // for loop useful to loop thorugh collection of variables
    let a = [2, 3, 4, 5, 6, 7];

    for elemetn in a.iter() {
        println!("the value of the is: {} ", elemetn);
    }

    for number in (1..4) {
        println!("{}", number)
    }

    // single line comments

    /*
    block comments
     */
}

// // creating the new functions
fn my_functions(x: i32, y: i32) -> i32 {
    println!("another function a x {} and y {}", x, y);

    let sum = x + y;

    // return sum; // 1 way of return
    // sum // another way of return

    x + y
}
