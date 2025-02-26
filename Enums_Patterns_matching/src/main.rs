// enum IpAddrKind {
//     V4,
//     V6,
// }


// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     let localhost = IpAddrKind::V4(127, 0, 0, 1);[]
// }

// fn route(ip_kind: IpAddrKind) {}


// enum IpAddrkind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// enum Message {
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn some_function() {
//         println!("I am writing rust code")
//     }
// }

// struct QuitMessage; // Unit struct 

// struct MoveMessage {
//     x : i32,
//     y: i32
// }

// struct WriteMessage(String); // Tuple struct 
// struct ChangeColorMessage(i32, i32, i32); // tuple struct


// fn main() {
//     enum Option<T> { // The Option require for optional value
//         Some(T), // here T is optionial using the Some keyword
//         None,
//     }
// }


// // Another Example
// fn main() {
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5); // Here y is an integer or none

//     let sum = x + y.unwrap_or(0); // Her the default value here is 0

//     println!("The sum is {}", sum);

// }




// Using the match expression

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }


fn  main() {
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("Three"),
        _ => (),
    }

    if let Some(3) = some_value {
        println!("Three");
    }
}