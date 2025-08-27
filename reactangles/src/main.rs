#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }
    fn width(&self) -> bool {
        self.width > 0
    }
    // fn can_hold(&self, other: &Rectangle) -> bool {
    //     self.width > other.width && self.height > other.height
    // }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 1,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle width has a nonzero width; it is {}, {}", rect1.width, rect1.height);
    }
    // let scale = 2;
    // let rect1 = Rectangle {
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };
    //
    // dbg!(&rect1);
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rect1.area()
        // area(&rect1)
    // );
    // println!("rect1 is {:?}", rect1)
}

// fn area (rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
