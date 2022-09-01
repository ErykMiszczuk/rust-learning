struct PointM<T, U> {
    x: T,
    y: U
}

impl<T, U> PointM<T, U> {
    fn mixup<T1, U1>(self, other: PointM<T1, U1>) -> PointM<T, U1> {
        PointM {
            x: self.x,
            y: other.y
        }
    }
}

struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// only point instances with f32 type will have this method
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in &list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = PointM { x: 6, y: 7.0 };
    let uni = PointM { x: 4, y: 2 };
    println!("integer.x = {}", integer.x());
    
    let p = uni.mixup(mixed);
    println!("p.x = {}, p.y = {}", p.x, p.y);

    // Monomorphization
    // There is no performance penalty with using generics because compiler 
    // creates concrete types from generics. It reads what types were used
    // on generics, and creates code for that type so it is replacing generic
    // definition into specific ones
}
