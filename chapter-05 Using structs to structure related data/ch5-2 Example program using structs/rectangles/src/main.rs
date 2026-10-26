struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // The normal way.
    let height = 50;
    let width = 30;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width, height)
    );

    // Using tuples
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle using tuples approach is {} square pixels",
        area_tuple(rect1)
    );

    // Refactoring with structs
    let rectangle = Rectangle {
        width: 20,
        height: 40,
    };
    println!(
        "The area of the rectangle using the structs is {}",
        rect_area_struct(&rectangle)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// calculate area using tuples
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// calculates area of rectangle using a struct
fn rect_area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
