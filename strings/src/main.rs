// fn main() {
//     // let data = "initial contents";
//     //
//     // let s = data.to_string();
//     //
//     // // The method also works on a literal directly:
//     // let s = "initial contents".to_string();
//
//     let s = String::from("initial contents");
//
//     println!("{s}");
// }

//
// fn main() {
//     let mut s1 = String::from("foo");
//     let s2 = "bar";
//     s1.push_str(s2);
//
//     println!("s1 is {s1}");
//     println!("s2 is {s2}");
// }
//
//
// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2;
//
//     println!("s3 is {s3}");
// }


fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}
