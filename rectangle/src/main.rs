struct Rectangle {
    height: i64,
    width: i64,
}

fn main() {
    let height: i64 = 10;
    let width: i64 = 20;
    let rectangle = Rectangle { height, width };

    let area: i64 = area(rectangle);

    println!("The area is {area}")
}

fn area(rectangle: Rectangle) -> i64 {
    return rectangle.width * rectangle.height;
}
