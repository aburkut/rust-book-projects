// fn main() {
//     let x = Some(5);
//     let y = 10;
//
//     match x {
//         Some(50) => println!("Got 50"),
//         Some(y) => println!("Matched, y = {y}"),
//         _ => println!("Default case, x = {x:?}"),
//     }
//
//     println!("at the end: x = {x:?}, y = {y}");
// }

// fn main() {
//     let x = 5;
//
//     match x {
//         1..=5 => println!("one through five"),
//         _ => println!("something else"),
//     }
// }

// fn main() {
//     let x = 'c';
//
//     match x {
//         'a'..='j' => println!("early ASCII letter"),
//         'k'..='z' => println!("late ASCII letter"),
//         _ => println!("something else"),
//     }
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let p = Point { x: 10, y: 11 };
//
//     let Point { x: a, y: b } = p;
//
//     assert_eq!(11, a);
//     assert_eq!(11, b);
// }

//
// fn main() {
//     let p = Point { x: 0, y: 7 };
//
//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {x}"),
//         Point { x: 0, y } => println!("On the y axis at {y}"),
//         Point { x, y } => {
//             println!("One neither axis: ({x}, {y})");
//         }
//     }
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     let msg = Message::ChangeColor(0, 160, 255);
//
//     match msg {
//         Message::Quit => {
//             println!("The Quit variant has no data to destructure");
//         },
//         Message::Move { x, y } => {
//             println!("Move in the x direction {x} and in the y direction {y}");
//         },
//         Message::Write(text) => {
//             println!("Text message: {text}");
//         },
//         Message::ChangeColor(r, g, b) => {
//             println!("Change color to red {r}, green {g} and blue {b}");
//         }
//     }
// }

// enum Color {
//     Rgb(i32, i32, i32),
//     Hsv(i32, i32, i32),
// }
//
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(Color),
// }
//
// fn main() {
//     let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
//
//     match msg {
//         Message::ChangeColor(Color::Rgb(r, g, b)) => {
//             println!("Change color to red {r}, green {g} and blue {b}");
//         }
//         Message::ChangeColor(Color::Hsv(h, s, v)) => {
//             println!("Change color to hue {h}, saturation {s} , value {v}")
//         }
//         _ => (),
//     }
// }

fn main() {
    let x = 4;
    let y = true;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
