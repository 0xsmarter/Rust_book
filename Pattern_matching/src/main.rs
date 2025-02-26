// #[derive(Debug)]
// enum Language {
//     English,
//     Spanish,
//     French,
//     German,
//     Russian,
// }

// fn main() {
//     let lang = Language::English;
//     match lang {
//         Language::English => println!("Hello"),
//         Language::Spanish => println!("Hola"),
//         Language::French => println!("Bonjour"),
//         Language::German => println!("Guten Tag"),
//         Language::Russian => println!("Privet"),
//         lang => println!("Unsupported language! {:?}", lang),
//     }

//     let authorization_status: Option<&str> = None;
//     let is_admin = false;
//     let group_id: Result<u8, _> = "34".parse();

//     if let Some(status) = authorization_status {
//         println!("authorization status: {}", status);
//     } else if is_admin {
//         println!("Authorization status: admin");
//     } else if let Ok(group_id) = group_id {
//         if group_id > 30 {
//             println!("Authorization status: privileged");
//         } else {
//             println!("Authorization status: basic");
//         }
//     } else {
//         println!("Authorization status: guest");
//     }


//     let mut stack = Vec::new();
//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     while let Some(top) = stack.pop() {
//         println!("{}", top);
//     }

//     let v = vec!['a', 'b', 'c'];

//     for (index, value) in v.iter().enumerate() {
//         println!("{} is at index {}", value, index);
//     }

//     let point = (3, 4);
//     print_coordinates(&point);
// }


// fn print_coordinates(&(x, y): &(i32, i32)) {
//     println!("Current position: ({}, {})", x, y);
// }




// Another example

fn main() {
    let x = 5;

    let x : Option<&str> = None;

    if let Some(x) = x {
        println!("Found a number: {}", x);
    }
}