struct Rectangle {
    height: i64,
    width: i64,
}

impl Rectangle {
    fn area(&self) -> i64 {
        return self.width * self.height;
    }
}

fn main() {
    let height: i64 = 10;
    let width: i64 = 20;
    let rectangle = Rectangle { height, width };

    let area: i64 = rectangle.area();

    println!("The area is {area}")
}
