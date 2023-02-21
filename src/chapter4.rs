pub fn chapter4() {
   // ownership for complex data type

   let mut s = String::from("foo");
   borrow(&mut s);

   take_ownership(s);
//    println!("Compile error due to change of ownership {}", s);

   let mut s2 = String::from("foo2");
   borrow2(&mut s2); // accept reference type

   take_ownership2(s2);

   let mut s3 = String::from("hellow");
   borrow3(&mut s3);
   take_ownership3(s3);
//    println!("Compile error {}", s3) // compile error
   
   
}

fn borrow(s: &mut String) { // accept a 'reference type'
    s.push_str(" , xxxxxxxxxx")
}

fn take_ownership(s : String) {
    println!("from take ownersip function:  {}", s)
}

fn borrow2(s : &mut String) {
    s.push_str(", xxxx2")

}

fn take_ownership2(s: String) {
    println!("ownership2 function: {}", s)
}

fn borrow3(s: &mut String){
    s.push_str("xxx3")
}
fn take_ownership3(s:String) {
    println!("Ownership function3 {}", s)
}