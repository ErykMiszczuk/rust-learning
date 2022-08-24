fn main() {
    let s = String::from("hello there");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    // println!("{}", s); that will not work because pointer was passed to
    // takes_ownership and was dropped there
    println!("{}", x);
}

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn makes_copy(int: i32) {
    println!("{}", int);
}
