use std::f64::consts::PI;

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),
    
}

fn calculate_area(shape:Shape)-> f64 {
    let area = match shape {
        Shape::Circle(radius)=> PI * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(length,width)=> length * width,
        };
        return area;
}
fn main() {
    let circle = Shape::Circle(9.1);
    let square = Shape::Square(4.4);
    let rect = Shape::Rectangle(6.0, 8.0);

    println!("The are of the Circle is {}", calculate_area(circle) );
    println!("The are of the Square is {}", calculate_area(square) );
    println!("The are of the Rectangle is {}", calculate_area(rect) );
}

