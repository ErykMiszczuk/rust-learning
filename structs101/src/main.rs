
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32); // also struct called tuple struct
struct Point(i32, i32, i32);

struct AlwaysEqual; // unit like structs without any fields
// usefull for implementing traits

fn build_user(email: String, username: String) -> User{
    // yes we can use function parameters in user factory
    // we just need same parameters names
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someoneusername"),
        active: true,
        sign_in_count: 42,
    };

    // To change field inside struct whole struct must be mutable
    user1.email = String::from("anotheremail@example.com");

    let email = String::from("admin@admin.net");
    let login = String::from("HouseMaster");

    let admin = build_user(email, login);

    println!("{}", admin.active);

    // struct update
    let admin2 = User {
        email: String::from("house@house.nl"),
        ..admin
    };
    // tha works very similar to spread operator in JS
    // but still rules about moving data still aplly

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Even thou those structs have the same data, 
    // the have diffrent types 'Color' and 'Point' respectively 
    // so there is no option to pass black as an argument 
    // to function that accepts points

    let subject = AlwaysEqual;
}
