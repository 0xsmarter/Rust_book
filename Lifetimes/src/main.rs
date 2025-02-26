// Degluing references 

// fn main() { // there is an error here
//     let r;

//     {
//         let x = 5;
//         r = &x;      
//     }   

//     println!("r: {}", r);
// }




// Another example 

// fn main() { // In this case there is no error 
//     let x = 5;

//     let r = &x;

//     println!("r: {}", r);
// }



// Another example




// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = String::from("xyz");

//     let result = longest(string1.as_str(), string2.as_str());

//     println!("The longest string is {}", result);
// }

// // &i32   // a reference 
// // &'a i32 // a reference with an explicit lifetime
// // &'a mut i32 // a mutable reference with an explicit lifetime


// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // Corrected the lifetime annotation
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }




// Another example 

// fn main() {
//     let string1 = String::from("abcd");

//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {}", result);
//     }
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }



// Another example

// fn main() {
//     let string1 = String::from("abcd");
//     let result;

//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {}", result);
//     }
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> String {
//     let result = String::from("really long string");
//     result
// }



// Another example

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// } 

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");

//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }






// Another example

// fn main() {}

// fn first_word<'a>(s: &'a str) -> &'a str {
//     let bytes = s.as_bytes();

//     for (i, &item) in  bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }



// ANother example

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }


// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }




// The static lifetime 

// fn main() {
//     let s: &'static str = "I have a static lifetime";
// }




// Combining traits with lifetime


use std::fmt::Display;

fn longest_with_an_annoucement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where T: Display, {
    println!("Annoucement! {}", ann);
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

fn main() {}