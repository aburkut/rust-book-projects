use crate::garden::vegetables::Asparagus;

pub mod garden;

fn test_func() {
    println!("Print test");
}

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    let some_var = 5;
    println!("{some_var}!");
    test_func();
}
