// method syntex is simillar to function. first parameter is always self

struct Rectangle {
    width: u32,
    height: u32
}

struct Point(i32, i32);


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width
    }

    fn max (self, other:Rectangle) -> Rectangle {
        Rectangle { width: self.width.max(other.width), height: self.height.max(other.height) }
    }


}

impl Rectangle {
    fn width(&self) -> bool {
        self.width  > 0
    }
}


impl Rectangle{
    fn square(size: u32) -> Self{
        Self { width: size
            , height: size}
    }
}

pub fn method_syntex() {

    impl Point {
        fn x(&self) -> i32 {self.0}
        
    };

    let p = Point(1,1);
    println!("Point x {}", p.x());
    let sq = Rectangle::square(5);
    sq.area();

    let rec1 = Rectangle{
        width : 10,
        height : 4
    };
    sq.width();
    let rec_other = Rectangle {
        width : 11,
        height: 5
    };


    let other_rec_result =     rec1.can_hold(&rec_other);

    println!("Other rectangle can hold result {}", other_rec_result);



    println!("The area of the rectangle is {} sqare pixels", rec1.area());

    if rec1.width() {
        println!("The rectangle width is true and area is {}", rec1.area());
    }

}