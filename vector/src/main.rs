
// fn main() {
//     let mut v = Vec::new();
//
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);
//
//     println!("{}, {}, {}, {}", v[0], v[1], v[2], v[3]);
// }


// fn main() {
//     let v = vec![1, 2, 3, 4, 5];
//
//     let third: &i32 = &v[2];
//     println!("The third element is {third}");
//
//     let second: Option<&i32> = v.get(1);
//
//     match second {
//         Some(second) => println!("The second element is {second}"),
//         None => println!("There is no second element"),
//     }
// }

fn main() {
    let mut v = vec![100, 32, 57, 100, 100500];

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }
}
