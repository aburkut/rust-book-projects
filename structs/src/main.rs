

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     println!("Hello, world!");

//     let mut user1 = User {
//         active: true,
//         username: String::from("user1"),
//         email: String::from("test@test.test"),
//         sign_in_count: 1,
//     };

//     user1.email = String::from("new_email@test.test");

//     println!("{0}, {1}", user1.email, user1.username);

//     let user2 = User {
//         email: String::from("another@example.com"),
//         username: String::from("user2"),
//         ..user1
//     };

//     println!("{0}, {1}", user2.email, user2.username);
// }

// fn buildUser(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }


// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }


struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}