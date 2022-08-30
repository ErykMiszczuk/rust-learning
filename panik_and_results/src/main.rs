use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// ? operator version
fn read_username_from_file_question() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn even_shorter_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
// ? can only be used in functions returning same type that code
// inside function body is returning
// Note that you can use the ? operator on a Result in a function that returns Result, 
// and you can use the ? operator on an Option in a function that returns Option,
// but you can’t mix and match. The ? operator won’t automatically convert a Result 
// to an Option or vice versa; in those cases, you can use methods like the ok method on Result 
// or the ok_or method on Option to do the conversion explicitly.

fn main() {
    // panic!("crash and burn")

    // let v = vec![1, 2, 3];

    // v[99];
    // accessing out of bounds data will panic to prevent buffer overreads exploit


    // Recoverable errors
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e)
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error)
            }
        }
    };

    // other options on handling errors
    let greeting_file_result = File::open("hello.txt").unwrap();
    // just panic!
    let greeting_file_result = File::open("hello.txt").expect("file should be included in project");
    // panic! but with nice error message

}
