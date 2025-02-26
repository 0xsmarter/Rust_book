// fn add_one(x: i32) -> i32 {
//     x + 1
// }

// fn do_twice<T>(f: T, arg: i32) -> i32
// where
//     T: Fn(i32) -> i32,
// {
//     f(arg) + f(arg)
// }

// fn main() {
//     let answer = do_twice(add_one, 5);
//     println!("The answer is: {}", answer);
// }

// Another example

// fn main() {
//     let list_of_numbers = vec![1, 2, 3];
//     let list_of_strings: Vec<String> = list_of_numbers.iter().map(|n| n.to_string()).collect();

//     println!("{:?}", list_of_strings);
// }




// Another example

// fn main() {
//     enum Status {
//         Value(u32),
//         Stop,
//     }

//     let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    
// }



// Another example

fn main() {}

fn returns_closure(a : i32) -> Box<dyn  Fn(i32) -> i32> {
    if a > 0 {
        Box::new(move |b| a + b)
    } else {
        Box::new(move |b| a - b)
    }
}