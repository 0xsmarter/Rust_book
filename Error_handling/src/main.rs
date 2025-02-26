// fn main() {
//     panic!("crash and burn"); 
// }



// ANother example 

// fn main() {
//     a();
// }

// fn a() {
//     b();
// }

// fn b() {
//     c(21);
// }

// fn c(num: i32) {
//     if num == 22 {
//         panic!("Don't pass in 22!");
//     }
// }


// ANother example 


// fn main() {
//     enum Result<T, E> {
//         Ok(T),
//         Err(E),
//     }
// }



// Another example


// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let f = File::open("hello.txt");

//     let _f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("Hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error)
//             }
//         }
//     };

//     let _f2 = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         }
//         else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }




// Another example 

// use std::fs::File;

// fn main() {
//     let _f = File::open("hello.txt").expect("Failed to open hello.txt");

//     // let _f2 = match _f {
//     //     Ok(file) => file,
//     //     Err(error) => panic!("Problem opening the file: {:?}", error),
//     // }
// }


// Another example : Error propagation

// use std::fs::{self, File};
// use std::io;
// use std::io::Read;
// use std::error::Error;


// fn read_username_from_file() -> Result<String, io::Error> {
//     // let mut s = String::new();
//     // File::open("open.txt")?.read_to_string(&mut s)?;

//     // Ok(s)

//     fs::read_to_string("open.txt")
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("open.txt")?;
//     Ok(())
// }


// Another example
use std::net::IpAddr;

fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}