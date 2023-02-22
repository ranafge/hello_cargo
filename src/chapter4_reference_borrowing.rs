use std::fmt::format;

pub fn reference_borrowing() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    greet(m1, m2);
    // compile error due ownership chage.
    // let s = format!("m1 is {} and m2 is {}", m1 , m1);
}

// greet function

fn greet(g1: String, g2: String) {
    println!("String one is {} and string two is {}", g1, g2)
}
