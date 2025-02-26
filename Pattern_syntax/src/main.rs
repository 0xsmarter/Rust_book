// fn main() {
//     let x = 1;

//     match x {
//         1 => println!("one"),
//         2 => println!("two"),
//         3 => println!("three"),
//         _ => println!("something else"),
//     }
// }

// fn  main() {
//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(50) => println!("Got 50"),
//         Some(y) => println!("Matched, y = {y}"),
//         _ => println!("Default case, x = {:?}, y = {y}", x),
//     }
// }

// fn main() {
//     let x = 1;

//     match x {
//         1 | 2 => println!("one or two"),
//         3 => println!("three"),
//         _ => println!("anything"),
//     }
// }

// Another example

// fn main() {
//     let x = 5;

//     match x {
//         1..=5 => println!("one through five"),
//         _ => println!("something else"),
//     }

//     let x = 'c';

//     match x {
//         'a'..='j' => println!("early ASCII letter"),
//         'k'..='z' => println!("late ASCII letter"),
//         _ => println!("something else"),
//     }
// }

// Another example

// struct Point {
//     y: i32,
//     x: i32,
// }

// // fn main() {
// //     let p = Point { x: 0, y: 7 };

// //     let Point { x , y  } = p;

// //     assert_eq!(0, x);
// //     assert_eq!(7, y);
// // }

// // Another example

// fn main() {
//     let p = Point { x: 0, y: 7 };

//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         Point { x: 0, y } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//     }
// }

// Another example

// enum Color {
//     Rbg(i32, i32, i32),
//     Hsv(i32, i32, i32),
// }

// enum Message {
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(Color),
// }

// fn main()  {
//     let msg = Message::ChangeColor(
//         Color::Hsv(0, 160, 255)
//     );

//     match msg {
//         Message::ChangeColor(Color::Rbg(r, g, b)) => {
//             println!(
//                 "Change color: red {}, green {}, blue {}",
//                 r, g, b
//             );
//         }

//         Message::ChangeColor(Color::Hsv(h, s, v)) => {
//             println!(
//                 "Change color: hue {}, saturation {}, value {}",
//                 h, s, v
//             );
//         }

//         _ => (),
//     }
// }

// Another example

// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let ((feet, inches), Point {x , y}) = ((3, 10), Point { x: 3, y: 10 });
// }

// Another example

// fn main() {
//     foo(3, 4);
// }

// fn foo(_: i32, y: i32) {
//     println!("this code only uses the y parameter: {}", y);
// }

// Another example

// fn main() {
//     let mut setting_value = Some(5);
//     let new_setting_value = Some(10);

//     match (setting_value, new_setting_value) {
//         (Some(_), Some(_)) => {
//             println!("Can't overwrite an existing customized value");
//         }

//         _ => {
//             setting_value  = new_setting_value;
//         }
//     }

//     println!("setting is: {:?}", setting_value);
// }

// Another example

// fn main() {
//     let numbers = (2, 4, 8, 16, 32);

//     match numbers {
//         (first, _, third, _, fifth) => {
//             println!("Some numbers: {first}, {third}, {fifth}")
//         }
//     }
// }

// Another example

// fn main() {
//     let _x = 5;
//     let y = 10;

//     let s = Some(String::from("hello!"));

//     if let Some(_) = s {
//         println!("found a string");
//     }

//     println!("{:?}", s);
// }

// Another example

// fn main() {
//     struct Point {
//         x: i32,
//         y: i32,
//         z: i32,
//     }

//     let origin = Point {x: 0, y: 0, z: 0};

//     match origin {
//         Point {x, ..} => println!("x is {}", x),
//     }
// }

// Another example

// fn main() {
//     fn main() {
//         let numbers = (2, 4, 8, 16, 32);

//         match numbers {
//             (first, .., last) => {
//                 println!("Some numbers: {first}, {last}")
//             }
//         }
//     }
// }

// Another example

// fn main() {
//     let num = Some(4);

//     match num {
//         Some(x) if x < 5 => println!("Less than five: {}", x),
//         Some(x) => println!("{}", x),
//         None => (),
//     }
// }

// Another example

// fn main() {
//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(n) if n == y => println!("Matched , n = {}", n),
//         _ => println!("Default case, x = {:?}, y = {}", x, y),
//     }
// }

// Another example

// fn main() {
//     let x = 4;
//     let y = false;

//     match x {
//         4 | 5 | 6 if y => println!("yes"),
//         _ => println!("no"),
//     }
// }

// Another example

enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),

        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }

        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
