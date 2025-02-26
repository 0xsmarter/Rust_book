use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Initial reference count: {}", Rc::strong_count(&a));

    // Create a new list that shares ownership of 'a'
    let b = Cons(3, Rc::clone(&a));
    println!("Reference count after creating b: {}", Rc::strong_count(&a));

    {
        // Create another list in a new scope
        let c = Cons(4, Rc::clone(&a));
        println!("Reference count after creating c: {}", Rc::strong_count(&a));
    }
    
    // After c goes out of scope, its reference is dropped
    println!("Reference count after c goes out of scope: {}", Rc::strong_count(&a));
}