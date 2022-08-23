fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    let y = 6; //statement
    // let x = y = 6 will not work in rust 
    // because assigment is not an expression

    // adding semicolon at the end of line turns that code into statement 
    // so this code block will make compiler angry at you (and we dont want it) 
    let yy = {
        let x = 3;
        x + 1
    };
    println!("The value of yy is {yy}");

    let xx = five();
    println!("The value of xx is {xx}");

    let six = plus_one(xx);
    println!("The value of six is {six}");

}

fn another_function(x: i32) {
    println!("Value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// functions returns last expresions by default
// so we can omit 'return' keyword
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}