struct Rectangle {
    height: i64,
    width: i64,
}

impl Rectangle {
    fn area(&self) -> i64 {
        return self.width * self.height;
    }
    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        let is_wider = self.width >= other_rectangle.width;
        let is_higher = self.height >= other_rectangle.height;
        return is_higher && is_wider;
    }
}

fn main() {
    let big_rectangle = Rectangle {
        height: 10,
        width: 10,
    };
    let small_rectangle = Rectangle {
        height: 10,
        width: 10,
    };

    println!("Area of big rectangle is {}", big_rectangle.area());
    println!("Area of small rectangle is {}", small_rectangle.area());

    print!(
        "Can big rectangle contain small rectangle? {}",
        big_rectangle.can_hold(&small_rectangle)
    );
}
