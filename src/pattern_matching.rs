enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // width, height
    Square(f64), // side
}

fn calculate_area(shape: &Shape) -> f64 {
    
    let ans = match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    };

    return ans;
}

fn main() {
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(4.0, 6.0);
    let square = Shape::Square(3.0);

    println!("Area of the circle: {}", calculate_area(&circle));
    println!("Area of the rectangle: {}", calculate_area(&rectangle));
    println!("Area of the square: {}", calculate_area(&square));
}