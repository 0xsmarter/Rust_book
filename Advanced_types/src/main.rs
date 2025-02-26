// use std::fmt;

// struct Wrapper(Vec<String>);

// impl fmt::Display for Wrapper {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }

// struct Age(u32);

// struct ID(u32);

// fn main() {
//     let w = Wrapper(vec!["hello".to_string(), "world".to_string()]);
//     println!("{}", w);
// }

// Another example

// fn main() {
//     type Kilometers = i32;

//     let x: i32 = 5;
//     let y: Kilometers = 5;

//     println!("x + y = {}", x + y);
// }

// Another example

// fn main() {

//     type Thunk = Box<dyn Fn() + Send + 'static>;

//     let f: Thunk = Box::new(|| println!("hi"));

//     fn takes_long_type(f: Thunk) {

//     }

//     fn returns_long_type() -> Thunk {
//         Box::new(|| println!("Hello from returned function"))
//     }
// }

// Another example

// fn main() {
//     while game_in_progress {
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//     }
// }

// Another example

// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//     }
// }


// Another example

// fn main() {
//     print!("forever");

//     loop {
//         print!("and ever");
//     }
// }



// Dynamical sized types 


// fn main() {
//     let s1: &str = "Hello there";
//     let s2: &str = "How's it going?";
// }

fn main() {}