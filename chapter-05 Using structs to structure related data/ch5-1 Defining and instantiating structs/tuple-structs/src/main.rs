struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // destructuring tuple struct
    let Point(x, y, z) = origin;
    println!("x = {x}, y = {y}, z = {z}");
}
