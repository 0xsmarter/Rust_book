// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }


// fn main() {
//     let user1 = User {
//         email: String::from("0xsmarter@gmail.com"),
//         username: String::from("0xsmarter"),
//         active: true,
//         sign_in_count: 1
//     };

//     let name = user1.username;
//     user1.username = String::from("Mike scott");

//     let user2 = build_user(email: String::from("0xsmarter@gmail.com"), username: String::from("Jojoruski"));


//     let user3 = User {
//         email: String::from("0xsmarter@gmail.com"),
//         username: String::from("Ruski"),
//         ..user2
//     };



// }


// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }


// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     struct Color(i32, i32, i32);

//     struct Point(i32, i32, i32);
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50
    };

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("rect: {:?}", rect);

    println!("The of the rectangle is {} square pixels. ", rect.area())
}

