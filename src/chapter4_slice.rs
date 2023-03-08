// slice is continuous element of a sequesce element.

pub fn slice() {
    let s = String::from("ayesha is a girl");
    println!("{}",first_word(s));
}

fn first_word(s: String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("Index is {} and item is {}", i, item);
        if item == b' ' {
            return  i;
        }

    }
    s.len()

}