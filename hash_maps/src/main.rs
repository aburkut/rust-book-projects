use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();
//
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 150);
//     scores.insert(String::from("Red"), 100500);
//
//     // let team_name = String::from("Blue");
//     // let score = scores.get(&team_name).copied().unwrap_or(0);
//     // println!("{score}");
//
//     for (key, value) in &scores {
//         println!("{key}: {value}");
//     }
// }


// fn main() {
//     let mut scores = HashMap::new();
//
//     scores.insert(String::from("Blue"), 150);
//     scores.insert(String::from("Blue"), 1000);
//     scores.insert(String::from("Yellow"), 11);
//     scores.insert(String::from("Red"), 17);
//
//     println!("{scores:?}");
// }

// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 1000);
//
//     scores.entry(String::from("Yelow")).or_insert(50);
//     scores.entry(String::from("Blue")).or_insert(50);
//
//     println!("{scores:?}");
// }


fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
