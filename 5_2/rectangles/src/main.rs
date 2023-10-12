#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn is_square(&self) -> bool {
        self.height == self.width
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn _double_height(&mut self) {
        self.height = self.height * 2
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rectangle: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let other_rectangle: Rectangle = Rectangle {
        width: 10,
        height: 10,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );

    if rectangle.can_hold(&other_rectangle) {
        println!("Rectangle can hold other rectangle");
    }

    let rect3: Rectangle = Rectangle::new(20, 30);

    let rect4: Rectangle = Rectangle::square(50);

    if rect3.is_square() {
        print!("rect3 is a squre");
    }

    if rect4.is_square() {
        println!("rect 4 is a square");
    }
}
