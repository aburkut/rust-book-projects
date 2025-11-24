// fn main() {
//     let s1 = String::from("Hello");
//     let s2 = s1.clone();
//     let s3 = s2.clone();

//     println!("s1 = {s1}, s2 = {s2}, s3 = {s3}!");
// }

// fn main() {
//     let s = String::from("hello");

//     takes_ownership(s);

//     let x = 5;

//     makes_copy(x);
// }

// fn takes_ownership(some_string: String) {
//     println!("{some_string}");
// }

// fn makes_copy(some_integer: i32) {
//     println!("{some_integer}");
// }

// fn main() {
//     let s1 = gives_ownership();

//     let s2 = String::from("hello");

//     let s3 = takes_and_gives_back(s2);

//     println!("{s1}, {s3}")
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");

//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }


// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}.");
// }

// fn calculate_length(s: &String) -> usize {
//     let length = s.len();

//     length
// }


// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
//     println!("{some_string}");
// }

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}