// bringing all public items defined in a path
use std::collections::*;

// why i can access 'front_of_house' even thou there is no 'pub'?
// because 'eat_at_restaurant' is sibling to 'front_of_house'
// but 'front_of_house' children are not accessible from 'eat_at_restaurant'
// without 'pub' keyword
mod front_of_house;

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order()
    }

    fn cook_order() {}
}

fn deliver_order() {}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // use shortcut
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// shortcut cannot be used to import two objects with same name eg. Result
// use std::fmt;
// use std::io;

// fn fun1() -> fmt::Result {
//     fmt::Result {}
// }

// fn fun2 -> io::Result<()> {
//     ()
// }
// But there is an option to rename same types to something else
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

// pub use crate::front_of_house::hosting;
// this makes hosting available to root module
// so access path is restaurant::hosting::add_to_waitlist()

// use std::cmp::Ordering;
// use std::io;
// For easier access we can use nested paths
// use std::{cmp::Ordering, io};

// for eg
// use std::io;
// use std::io::Write;
// is equal to
// use std::io::{self, Write};
