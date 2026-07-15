#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// All functions defined within an impl block are called associated functions.
impl Rectangle {
    // &self = self: &Self.
    // &self as we don't want to take ownership, just borrow it.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rec1.area()
    );

    if rec1.width() {
        println!("The rectangle has a nonzero width; it is {}", rec1.width);
    }
    /* 
    When you call a method with object.something(),
    Rust automatically adds in &, &mut, or * so that object matches the signature 
    of the method. In other words, the following are the same:
    p1.distance(&p2);
    (&p1).distance(&p2);
    */

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rec1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rec1.can_hold(&rect3));

    let sq = Rectangle::square(3);
}
