fn main() {
    
    // loop {
    //     println!("loops infinity")
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loop labels
    // loop labels starts with ' and 'break' and 'continue' can stop
    // specific labeled loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    
    // for loop
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is {element}");
    }
    
    // range based for loop 
    for num in (1..4).rev() {
        println!("{num}");
    }
    println!("RANGED LIFTOFF!!!");

}
