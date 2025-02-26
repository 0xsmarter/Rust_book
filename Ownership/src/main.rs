// fn main() {
//     // --- Ownership rules ---
//     // 1 - Each value in Rust has a variable that's called its owner.
//     // 2 - There can be only one onwer at a time
//     // 3 - When the owner goes out of scope , the value will be dropped

//     { // s is not valid there , it's not yet declared
//         let s = String::from("hello"); // s is valid from this point forward
//         // Here we do stuff with s
//     } // this scope is now over , and s is no longer valid
// }


// fn main() {
//     let x = 5;
//     let y = x; // Copy

//     let s1 = String::from("Hello");
//     let s2 = s1.clone();

//     println!("{}, world!", s1);
// }

// fn main() {
//     let s = String::from("hello");
//     takes_ownership(s.clone());
//     println!("{}", s);

//     let x = 5;
//     makes_copy(x);
//     println!("{}", x);

// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }


// fn main() {
//     let s1 = gives_ownership();
//     let s2 = String::from("hello");
//     let s3 = takes_and_gives_back(s2);
//     println!("s1 = {}, s3 = {}", s1, s3);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");

//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }



// fn main() {
//     let mut s1 = String::from("hello");
//     change(&mut s1);

// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }



// fn main() {
//     let mut  s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;

//     println!("{} , {}", r1, r2);

//     let r3 = &mut s;
//     println!("{}", r3);
// }



// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }


// fn main() {
//     let mut s = String::from("hello world");
//     let s2 = "Hello world";

//     let word = first_word(s2);
// }



// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }


fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];

}