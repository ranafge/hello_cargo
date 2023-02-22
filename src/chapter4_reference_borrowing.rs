use std::{fmt::format, string};

pub fn reference_borrowing() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    greet(m1, m2);
    // compile error due ownership chage.
    // let s = format!("m1 is {} and m2 is {}", m1 , m1);

    // returning ownership
    let string1 = String::from("Hello");
    let string2 = String::from("World");
    let (g1_again, g2_again) = return_ownership(string1, string2);
    let s = format!("{}-{}", g1_again, g2_again);
    println!("sssss-{s}");
    // reference is a kind of pointer.

    let take_str_one = String::from("WASIR");
    let take_str_two = String::from("ISLAM");
    reference_example(&take_str_one, &take_str_two);
}

// greet function

fn greet(g1: String, g2: String) {
    println!("String one is {} and string two is {}", g1, g2)
}

fn return_ownership(g1: String, g2: String) -> (String, String) {
    println!("String1-{} and String2-{}", g1, g2);
    (g1, g2)
}

fn reference_example(take_string_one: &String, take_string_two: &String) {
    println!(
        "Take string one {} Take string two {}",
        take_string_one, take_string_two
    )
}
