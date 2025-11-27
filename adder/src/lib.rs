// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//
//         let result = add(100, 15);
//         assert_eq!(result, 115);
//
//         let result = add(100500, 500);
//         assert_eq!(result, 101000);
//     }
//
//     #[test]
//     fn another() {
//         panic!("Make this test fail");
//     }
// }


// #[derive(Debug)]
// pub struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//
//         let smaller = Rectangle {
//             width: 5,
//             height: 5,
//         };
//
//         assert!(larger.can_hold(&smaller));
//     }
//
//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };
//
//         assert!(!smaller.can_hold(&larger));
//     }
// }


// fn add_two(a: u64) -> u64 {
//     a + 2
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_adds_two()  {
//         let result = add_two(10);
//         assert_eq!(result, 12);
//     }
// }

//
// pub struct Guess {
//     value: i32,
// }
//
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {value}.")
//         }
//
//         Guess { value }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     #[should_panic(expected = "Guess value must be between 1 and 100")]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
//
//     #[test]
//     fn correct_val() {
//         let result = Guess::new(99);
//         assert_eq!(result.value, 99);
//     }
// }


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
