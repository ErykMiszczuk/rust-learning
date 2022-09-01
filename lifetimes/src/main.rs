use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let string1 = "abcd".to_string();
    let result;
    {
        let string2 = "xyz".to_string();
    
        result = longest(string1.as_str(), string2.as_str());
        println!("The longes string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime annottation syntax
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// Lifetime elision
// In some situations compiler can predict lifetime so that is why there isn't always
// a need to add them. If rust compiler is not sure how lifetime can the be added 
// to your code it wont compile and ask user to provide required data.
// Every new version of tooling new patterns might be added so some code might not need
// lifetime annotations anymore.

// Full definition
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}