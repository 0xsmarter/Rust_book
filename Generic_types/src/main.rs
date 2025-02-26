
// An example of reducing code duplication

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let largest = get_largest(number_list);

//     println!("The largest number is {}", largest);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let largest = get_largest_char(char_list);

//     println!("The largest number is {}", largest);
// }

// fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     largest
// }

// fn get_largest_char(number_list: Vec<char>) -> char {
//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     largest
// }





// Another example 

// struct Point<T, U> { // T and U are  the generic type 
//     x: T,
//     y: U,
// }

// fn main() {
//     let _p1 = Point {x: 5, y: 10};
//     let _p2 = Point {x: 5.0, y: 10.0};
//     let _p3 = Point {x: 5, y: 10.0};

// }



// Generics inside enums 

// fn main() {
//     enum Option<T> {
//         Some(T),
//         None,
//     }

//     enum Result<T, E> {
//         Ok(T),
//         Err(E)
//     }
// }



// Using generics on structs 

// struct Point<T> {
//     x: T,
//     y: T
// }

// impl<U> Point<U> {
//     fn x(&self) -> &U {
//         &self.x
//     }
// } 

// impl Point<f64> {
//     fn y(&self) -> f64 {
//         self.y
//     }
// }

// fn main() {
//     let p = Point {x: 5, y : 10};
//     let p1 = Point {x: 5.0, y: 10.0};
// }



// Another example 

// struct Point<T, U> {
//     x: T,
//     y: U
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c' };

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }




// Another example 

enum Option<T> {
    Some(T),
    None,
}


fn main() {
    let integer = Option::Some(5);
    let float = Option::Some(5.0);
}