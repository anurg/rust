enum Shape {
    Rectangle(f64, f64), //width,height
    Circle(f64),         //radius
}
fn main() {
    let rec: Shape = Shape::Rectangle(10.0, 15.0);
    println!("Recatangle Area: {} ", calaculate_area(rec));

    let circle = Shape::Circle(5.0);
    println!("Recatangle Area: {} ", calaculate_area(circle));

    fn calaculate_area(shape: Shape) -> f64 {
        match shape {
            Shape::Rectangle(width, height) => width * height,
            Shape::Circle(radius) => 3.14 * radius * radius,
        }
    }
}
