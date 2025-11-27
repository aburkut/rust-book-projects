
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }
//
// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";
//
//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {result}");
// }


// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
//
// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().unwrap();
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
//
//     println!("{}", i.part);
// }

//
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
//
// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
// }
//
// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {announcement}");
//         self.part
//     }
// }
//
// fn main() {}


use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let str1 = String::from("abcd");
    let str2 = String::from("asasas");
    let ann = String::from("ann");
    longest_with_an_announcement(&str1, &str2, ann);
}
