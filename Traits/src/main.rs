// // use std::fmt::format;
// // use std::fmt::Display;
// // use std::fmt::Debug;
// pub struct NewsArticle {
//     pub author: String,
//     pub headline: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize_author(&self) -> String {
//         format!("{}", self.author)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// // Implement the Summary trait for Tweet
// impl Summary for Tweet {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }

//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// // pub trait Summary {
// //     fn summarize_author(&self) -> String;

// //     fn summarize(&self) -> String {
// //         format!("(Read more from {} ...)", self.summarize_author())
// //     }
// // }

// // pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary) {
    
// // }

// // pub fn notifyb<T: Summary + Display>(item1: &T, item2: &T) {}

// // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// //     0 // Return an i32 value
// // }

// // fn other_function<T, U>(t: &T, u: &U) -> i32 
// //     where T: Display + Clone,
// //     U: Clone + Debug,
// // {}

// // fn main() {
// //     let tweet = Tweet {
// //         username: String::from("@johndoe"),
// //         content: String::from("Hello world"),
// //         reply: false,
// //         retweet: false
// //     };

// //     let article = NewsArticle {
// //         author: String::from("John Doe"),
// //         headline: String::from("The Sky is falling"),
// //         content: String::from("The is not actually falling.")
// //     };

// //     notify(&article);

// //     println!("Tweet summary: {}", tweet.summarize());
// //     println!("Article summary: {}", article.summarize());
// // }



// // Return types 

// fn main() {
//     let switch = true; // or false, depending on your logic
//     println!("{}", returns_summarizable(switch).summarize());
// }

// pub trait Summary {
//     fn summarize_author(&self) -> String;

//     fn summarize(&self) -> String {
//         format!("(Read more from {} ...)", self.summarize_author())
//     }
// }

// fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
//     if switch {
//         Box::new(NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             author: String::from("Of course, as you probably already know, people"),
//             content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL"),
//         })
//     } else {
//         Box::new(Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("Of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         })
//     }
// }





// ANother exmaple 


// use std::fmt::Display;

// struct Pair<T> {
//     x: T,
//     y: T,
// }


// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }

// }

// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest number is x = {}", self.x);
//         } else {
//             println!("The largest number is y = {}", self.y);
//         }
//     }
// }

// fn main() {}





// Another example 

use std::fmt::Display;

impl<T: Display> ToString for T {
    
} 

fn main() {}