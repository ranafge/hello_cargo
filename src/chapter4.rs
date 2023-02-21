pub fn chapter4() {
   // ownership for complex data type

   let mut s = String::from("foo");
   borrow(&mut s);

   take_ownership(s);
   

   
}

fn borrow(s: &mut String) { // accept a 'reference type'
    s.push_str("xxxxxxxxxx")
}

fn take_ownership(s : String) {
    println!("from take ownersip function:  {}", s)
}