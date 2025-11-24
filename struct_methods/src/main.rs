struct Rectanle {
    width: u32,
    height: u32,
}

impl Rectanle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn print_area(&self) {
        println!(
            "The area of the rectangle is {} square pixels.",
            self.area(),
        )
    }

    fn can_hold(&self, other: &Rectanle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// fn main() {
//     let rect1 = Rectanle {
//         width: 30,
//         height: 50,
//     };

//     rect1.print_area();
//     if rect1.width() {
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }
// }


fn main() {
    let rect1 = Rectanle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectanle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectanle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectanle::square(100);
    println!("sq width: {}, height: {}", sq.width, sq.height);
}