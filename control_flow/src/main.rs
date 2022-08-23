fn main() {
    let number = 6;

    // condition must be a bool, unlike in js where
    // variable will be coerced to bool
    if number < 5 {
        println!("TRUE!");
    } else {
        println!("FALSE");
    }

    if number != 0 {
        println!("Number was something else than zero")
    }

    if number % 4 == 0 {
        println!("number divisible by 4")
    } else if number % 3 == 0 {
        println!("number divisible by 3")
    } else if number % 2 == 0 {
        println!("number divisible by 2")
    } else {
        println!("number not divisible by 4, 3, 2")
    }

    // ternary expression
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number if {number}");
}
