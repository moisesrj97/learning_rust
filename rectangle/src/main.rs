fn main() {
    let height: i64 = 10;
    let width: i64 = 20;
    let dimensions = (height, width);

    let area: i64 = area(dimensions);

    println!("The area is {area}")
}

fn area(dimensions: (i64, i64)) -> i64 {
    return dimensions.0 * dimensions.1;
}
