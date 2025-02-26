// pub trait Iterator<T> {

//     fn next(&mut self) -> Option<T>;
// }

// struct Counter {}

// impl Iterator<u32> for Counter {

//     fn next(&mut self) -> Option<u32> {
//         Some(0)
//     }
// }

// impl Iterator<u16> for Counter {

//     fn next(&mut self) -> Option<u16> {
//         Some(1)
//     }
// }

// fn main() {}

// use std::ops::Add;

// #[derive(Debug, PartialEq)]

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Add for Point {
//     type Output = Point;

//     fn add(self, other: Point) -> Point {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(
//         Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
//         Point { x: 3, y: 3 }
//     );
// }

// Another example

// use std::ops::Add;

// struct Millimeters(u32);

// struct Meters(u32);

// impl Add<Meters> for Millimeters {
//     type Output = Millimeters;

//     fn add(self, other: Meters) -> Millimeters {
//         Millimeters(self.0 + (other.0 * 1000))
//     }
// }

// fn main() {}

// Another example

// trait Pilot {
//     fn fly(&self);
// }

// trait Wizard {
//     fn fly(&self);
// }

// struct Human;

// impl Human {
//     fn fly(&self) {
//         println!("waving arms furiously");
//     }
// }

// impl Pilot for Human {
//     fn fly(&self) {
//         println!("This is your captain speaking")
//     }
// }

// impl Wizard for Human {
//     fn fly(&self) {
//         println!("Up!")
//     }
// }

// fn main() {
//     let person = Human;
//     person.fly();
//     Pilot::fly(&person);
//     Wizard::fly(&person);
// }




// Another example
// use std::fmt;

// trait OutlinePrint: fmt::Display {
//     fn outline_print(&self) {
//         let output = self.to_string();
//         let len = output.len();
//         println!("{}", "*".repeat(len + 4));
//         println!("*{}*", " ".repeat(len + 2));
//         println!("* {} *", output);
//         println!("*{}*", " ".repeat(len + 2));
//         println!("{}", "*".repeat(len + 4));
//     }
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl OutlinePrint for Point {

// }

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

// fn main() {}


// Another example

use std::fmt;


struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec!["hello".to_string(), "world".to_string()]);
    println!("{}", w);
}