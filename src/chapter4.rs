pub fn chapter4() {
   // ownership for complex data type

   let s = String::from("foo");
   borrow(&s);

   
}

fn borrow(s: &String) { // accept a 'reference type'
    println!("s: {}", s);
}
