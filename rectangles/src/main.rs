// Simple naive way
// fn main() {
//     println!("Hello, world!");

//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} pixels",
//         area(width1, height1)
//     )
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Using tuples, adds structure but is now confusing because the fields are now index-dependent.
// If we wanted to render a rectangle on the screen, you would have to keep in mind which
// was the width and which the height
// fn main() {
//     let dimensions = (30, 50);
//     println!("The area of the rectangle is {} pixels", area(dimensions));    
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Using structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }    
    // You can even implement methods with the same name as one of the struct's fields
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // This is called an associated function and not a method because it does not reference &Self.
    // Accessing it is a litle bit different, you must use the :: syntax
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    // the dbg! macro returns ownership of the value back to rectangle
    let rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let rectangle2 = Rectangle {
        width: 5,
        height: 10,
    };

    let rectangle3 = Rectangle {
        width: 90,
        height: 75,
    };

    println!("The area of the rectangle is {} pixels", rectangle.area());
    println!("Is the width of rectangle nonzero? {}", rectangle.width());
    println!("The rectangle {:#?}", rectangle);
    println!("Can rect 1 hold rect 2? {}", rectangle.can_hold(&rectangle2));
    println!("Can rect 1 hold rect 3? {}", rectangle.can_hold(&rectangle3));

    let square = Rectangle::square(32);
    println!("Here is a square! {:#?}", square);

}

// fn area(r: &Rectangle) -> u32 {
//     r.width * r.height
// }
