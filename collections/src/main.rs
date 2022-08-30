use std::collections::HashMap;

fn main() {
    // rust cannot infer type so we have to provide it by hand
    let v: Vec<i32> = Vec::new();

    // vectors are so popular so there is a macro for creating it
    let v = vec![1, 2, 3];

    // same rules apply - to modify v we need to add that this variable is mutable
    // suprisingly here rust will now what data we will pass to vector,
    // but persoally i would add type annotation just for readability
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Oh and one more thing - shadowing variables works, as shown above

    // Accessing elements
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }

    // Iterating over vector
    for i in &v {
        println!("{}", i)
    }

    // To make changes we should have mutable vector and mutable reference
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50
    }
    // * dereference operator - used to get real value

    // To store multiple types of data inside vector we can use diffrent varants of enums
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    // STRING
    let mut s = String::new();

    let data = "initial contents 🔥";

    let s = data.to_string();

    let s = "initial contents 🔥".to_string();

    let s = String::from("initial contents 🔥");
    // every string is utf8 🔥
    // string concatenation + operaor or format! macro
    let mut s = String::from("Hello there");
    s.push_str("General Kenobi");

    let mut u = String::from("Unlimited ");
    let p = "power!";
    u.push_str(p);
    println!("{}", p); // p is still available!

    let mut p = String::from("p");
    p.push('p'); // push only accepts characters

    // Concatenation
    let s1 = String::from("Kansei ");
    let s2 = String::from("dorifto");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

    // using format macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // Iterating
    // Note there is no indexing access for string because rust
    // strings are utf8 so can contain diactricts, wich on its own
    // doesn't make sense

    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }

    // Hash maps
    let mut scores = HashMap::new();

    scores.insert("Red".to_string(), 10);
    scores.insert("Blue".to_string(), 50);

    // Accessing
    let team_name = "Blue".to_string();
    let score = match scores.get(&team_name) {
        Some(score) => score,
        None => &0,
    };

    // Iterating
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // typical rules of ownership apply to hashmap aswell

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    scores.entry("Red".to_string()).or_insert(50);
    scores.entry("Blue".to_string()).or_insert(50);

    println!("{:?}", scores);

    // Updating a value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
