// use std::ops::Add;
// #[derive(Debug, Copy, Clone, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// impl Add for Point {
//     type Output = Point;
//
//     fn add(self, other: Point) -> Point {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }
//
// fn main() {
//     assert_eq!(
//         Point { x: 1, y: 2 } + Point { x: 5, y: 6 },
//         Point { x: 6, y: 8 },
//     );
// }

// trait Pilot {
//     fn fly(&self);
// }
//
// trait Wizard {
//     fn fly(&self);
// }
//
// struct Human;
//
// impl Pilot for Human {
//     fn fly(&self) {
//         println!("This is your captain speaking.");
//     }
// }
//
// impl Wizard for Human {
//     fn fly(&self) {
//         println!("Up!");
//     }
// }
//
// impl Human {
//     fn fly(&self) {
//         println!("*waving arms furiously*");
//     }
// }
//
// fn main() {
//     let human = Human {};
//     human.fly();
//     Pilot::fly(&human);
//     Wizard::fly(&human);
// }

// use std::fmt;
// use std::fmt::write;
//
// trait OutlinePrint: fmt::Display {
//     fn outline_print(&self) {
//         let output = self.to_string();
//         let len = output.len();
//         println!("{}", "*".repeat(len + 4));
//         println!("*{}*", " ".repeat(len + 2));
//         println!("* {output} *");
//         println!("*{}*", " ".repeat(len + 2));
//         println!("{}", "*".repeat(len + 4));
//     }
// }
//
//
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }
//
// impl OutlinePrint for Point {}
//
// fn main() {
//     let point = Point { x: 10, y: 12 };
//     point.outline_print();
// }


use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
