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

    // Dereferencing a pointer access its data
    // dereference operator [*]

    let mut x = Box::new(1);
    let a = *x; // 1
    *x += 1; // x = 2

    let r1 = &x; // point to the x reference
    let b = **r1; // pint to the heap value directly

    let r2 = &*x; // point to the heap vlaue

    let c = *r2; // c = 2

    let  my_string = String::from("Sharmin");
    let string_length = my_string.len(); // implicit reference
    let my_string_length = str::len(&my_string);
    println!("My string length is {}", string_length);
    println!("My string length_two is {}", my_string_length);

    // reference is read-only
    // immutable references is call shared references
    let mut e = String::from("hello");
    let e2: &mut String = &mut e;
    println!("e2 e2 {}", e2);
    e2.push_str(" world Rana");
    println!("e2 push string - {}", e2);

    let mut z = 1;
    let w = &z;
    println!("www is {}", w);
    
    

} // end reference_borrowing function

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
