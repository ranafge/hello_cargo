use std::io;
mod chapter4;
// main function
fn main() {
    // print a string
    println!("Hello, world!");
    // sign integer
    let x = -3;
    // printing sign integer
    println!("x is : {}", x);
    // own scope
    {
        let x = 2;
        println!("x is {}", x);
    };
    println!("This is {}", x);
    let y = 43;
    print!("x + y is : {} ", x + y);
    const MINUTE_IN_SECOND: u32 = 60;
    print!("minute in second is {}", MINUTE_IN_SECOND);

    let floating_point: f32 = 2.22222;
    println!("Floating point number is : {}", floating_point);
    // initalization boolean type
    let true_or_false: bool = false;
    print!("True or flase is {}", true_or_false);
    // initalization char type
    let character: char = ';';
    println!("char is : {}", character);
    // initalization tuple
    let tup: (i32, bool, char) = (33, false, 'r');
    // printing tuple
    print!("Tubel is {} {} {}", tup.1, tup.0, tup.2);
    // initalization of array (mutable)

    let mut arr = [1, 2, 3, 344, 5, 66];
    arr[0] = 10;
    // printing a array element
    print!(" array {}", arr[0]);

    // Printing element using for loop

    for ele in arr {
        println!("Array element is {}", ele);
    }

    // user input

    let mut input = String::new();
    println!("Enter a string ");
    io::stdin()
        .read_line(&mut input)
        .expect("fail to read line");
    println!("User input is {}", input);

    // arithmetic and type casting

    let x: f32 = 10.0; // 0-255
    let y: f64 = 10.0; // -128-127

    let z = x as f64 + y;
    print!("total of x and y is :{}", z);

    // mod
    let a = 100;
    let b = 10;
    print!("Modulas {}", a % b);

    // overflow
    let over_flow: u8 = 255; // 0-255
                             // print!("OVERFLOW ERROR MESSAGE {}", over_flow +1);
    print!("OVERFLOW ERROR MESSAGE {}", over_flow);

    // type casting

    let mut input2 = String::new();
    println!("Enter number : ");
    io::stdin().read_line(&mut input2).expect("FAIL TO READ");
    let int_input: i64 = input2.trim().parse().unwrap();
    print!("{}", int_input + 2);

    // condition if /else

    let cond = (2 as f32) < 2.2;

    let cond2 = true && cond;
    let cond3 = true || cond;

    println!(" Conditional result: {}", cond2);
    println!("Conditional result for OR {}", cond3);

    let food = "cookies";

    if food == "cookies" {
        println!("I like to cookies");
    } else if food == "fruit" {
        println!("I likde fruit also");
    } else {
        println!("I do not like cookies");
    }

    replace();

    // struct is way to create a complex data type.
    create_complex_data_type();
}

// string method

fn replace() {
    let hello = String::from("Hello world");

    println!("Hello string is : {hello}");
}

// struct way to create complex data type

struct Colour(u32, u32, u32);
fn create_complex_data_type() {
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    let mut bg = Color {
        red: 255,
        green: 35,
        blue: 160,
    };
    bg.red = 150;
    bg.blue = 150;
    bg.green = 150;
    println!("Color is {} {} {}", bg.red, bg.green, bg.blue);

    struct Point {
        x: u128,
        y: u128,
    }

    let coordinate = Point { x: 223, y: 33 };
    println!("coordinate is x:{}, y:{}", coordinate.x, coordinate.y);

    let red = Colour(200, 100, 50);
    println!("Color is {} , {}, {}", red.0, red.1, red.2);

    // Ownership chapter4

    // safety is absence of undefined behavior

    // String is a type does not impletement the Copy trait.

    let name = String::from("Rust");

    do_something2(&name);
    println!("{name}");

    fn do_something2(name: &str) {
        println!("Hello {} ", name);
    }

    #[derive(Debug)]
    struct Movie {
        title: String,
    }

    let movie = Movie {
        title: String::from("Rust"),
    };
    do_something3(movie);

    fn do_something3(movie: Movie) {
        println!("Movide : {:?}!", movie);
    }

    chapter4::chapter4();

    
}
