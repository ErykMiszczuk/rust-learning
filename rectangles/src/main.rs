#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 30;
    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );
    
    // tuple version
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area_tup(rect1)
    );

    // struct version
    let rect2 = Rectangle {
        width: 30,
        height: 30
    };
    let rect3 = Rectangle {
        width: 20,
        height: 15
    };
    println!("rect2 is {:?}", rect2);
    println!(
        "The area of the rectangle is {} square pixels",
        rect2.area()
    );
    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect3));
}

// fn area_str(rectangle: &Rectangle) -> i32 {
//     rectangle.width * rectangle.height
// }

fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: i32, height: i32) -> i32 {
    width * height
}
