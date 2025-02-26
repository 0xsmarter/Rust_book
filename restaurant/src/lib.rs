// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//     }

// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }


// Another example 

// fn serve_order() {
// }

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }



// Another example 

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");

//     meal.toast = String::from("Wheat");
// }



// Another example 

// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad:
// }




// Another example 

//  Here we import all of them at once 

use rand::{Rng, CryptoRng, ErrorKind::Transient}; // Import all the modules at once 

use std::io::{self, Write}; // We import both self and Write at the same time 

use std::io::*; // Here we import import all items under io


mod front_of_house; 

pub use self::front_of_house::hosting; // the use keyword help allow us to easily call the "front_of_house" module and put in scope so we don't have to call it himeself

pub fn eat_at_restaurant() {

    let secret_number = rand::thread_rng().gen_range(1, 101);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}




// Another example 


// use std::fmt;
// use std::io::Result as IoResult;

// fn function() -> Result {
//     Ok(())
// }

// fn function2() -> IoResult<()> {
//     Ok(())
// }