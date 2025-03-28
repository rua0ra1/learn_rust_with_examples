// fn main() {
//     // stack is the fixed memory where program have access to
//     // it can't grow and data is stored in the stack ( LIFO)
//     // the size of the stack is computed at compile time ( we should know the size of the variable at compile time)
//     // heap is less organized, the data is dynamic size and we control the life of the data

//     /* ------ ownership rules
//      1) each value in rust has a vriable that called its owner
//      2) there can only be one owner at a time
//      3) when the owner goes out of scope the value will be fropped
//     */
//     // {
//     //     // s is not vlaid here, its not yet declared
//     //     let _s = String::from("hello"); // s is valid from this point forward
//     // } // this scope is over and s is no longer valid

//     let x = 5;
//     let y = x; // copy of the x

//     let s1 = String::from("hello");
//     //let s2 = s1; // move (not shallow copy) s1 is not valid

//     let s2 = s1.clone();

//     let s3 = s2;

//     print!("the value is {}", s1);
//     print!("the value is {}", s2);
// }

/*

// take and give the owner ship through funcitons using move
fn main() {
    let s = String::from("hello");
    takes_ownership(s);

   // print!("{}", s);

    let x = 5;
    makes_copy(x);
    print!("{}", x);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn takes_ownership(some_string: String) {
    print!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    print!("{}", some_integer);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

*/

// take the value of the string without ownership using references
// passing the value without ownership is called borrowing( we are pointing the owner no the value)
// fn main() {
//     let s1 = String::from("hello");

//     let len = caluclate_length(&s1);
//     println!("the length of '{}' {} ", s1, len);
// }

// fn caluclate_length(s: &String) -> usize {
//     s.push_str("change the value"); // we can't do it as it immutable
//     let length = s.len();
//     length
// }

// mutable references
// rules of the mutable references
// 1) you can only have one mutable references for a particular data in the current scope
// 2) using this rule, rust avoids data racing when two pointers are pointing to two data. when both are trying to read and write at the same time

// fn main() {
//     let mut s1 = String::from("hello");

//     change(&mut s1)
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(",world");
// }

// fn main() {
//     let mut s1 = String::from("hello");

//     println!(" the value of the s1 is {} ", s1);

//     let s2 = &mut s1;

//     s2.push_str("no hello ");

//     println!(" the value of the s1 is {} ", s1);
//     println!(" the value of the s1 is {} ", s2);
// }

// two immutable referneces, in this case they just read the data cant write it
// you can't have mutable reference too immutable variable

// you can't have immutable mutable active at the same time

// note that scope of the reference ends with where it is used last time.
// fn main() {
//     let mut s1 = String::from("hello");

//     let r1 = &s1;
//     let _r3 = &s1;

//     let _r2 = &mut s1;

//     println!(" the value of the r1 {}", r1)
// }

/* // dangling refereces: when we have reference to invalid memory or data

// the main rules of references

1. at any given time, you can either one mutable reference
2. or any number of immutable reference
3. reference must be always valid

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
 */

//// slices /////////////
/// 1) the slices let you reference a continugous series of elements with in a data without
/// entire collection
/// 2) slices wont take the ownership of the underlying data

fn main() {}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
