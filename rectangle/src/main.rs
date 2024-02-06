fn main() {
    let height: i64 = 10;
    let width: i64 = 20;

    let area: i64 = area(height, width);

    println!("The area is {area}")
}

fn area(height: i64, width: i64) -> i64 {
    return height * width;
}
