### Rust Notes

### Rust Variables

### functions

### Rust Control Statements

### Structs & Ijmplementation

```
fn main() {
    struct Rect {
        width: i32,
        height: i32,
    }
    let rec = Rect {
        width: 15,
        height: 10,
    };
    impl Rect {
        fn area(&self) -> i32 {
            self.width * self.height
        }
        fn permimeter(&self) -> i32 {
            return (self.width + self.height) * 2;
        }
    }
    let area: i32 = rec.area();
    println!(
        " The Area of Rectanngle with width: {}, height: {} is {}",
        rec.width, rec.height, area
    );
    println!(
        " The Permimeter of Rectanngle with width: {}, height: {} is {}",
        rec.width,
        rec.height,
        rec.permimeter()
    );
}

```

### Enums
```
fn main() {
    enum Directions {
        UP,
        DOWN,
        RIGHT,
        LEFT,
    }
    let mut movement: Directions = Directions::UP;

    match movement {
        Directions::UP => println!("You are moving UP."),
        Directions::DOWN => println!("You are moving DOWN."),
        Directions::RIGHT => println!("You are moving RIGHT."),
        Directions::LEFT => println!("You are moving LEFT."),
    }
}
```

- Another Enum example
```
enum Shape {
    Rectangle(f64, f64), //width,height
    Circle(f64),         //radius
}
fn main() {
    let rec: Shape = Shape::Rectangle(10.0, 15.0);
    // println!("Recatangle Area: {} ",calaculate_area(rec));
    calaculate_area(rec);
    let circle = Shape::Circle(5.0);
    calaculate_area(circle);
    
    fn calaculate_area(shape: Shape) -> f64 {
        match shape {
            Shape::Rectangle(width, height) => {
                let area: f64 = width * height;
                println!("Rectangle Area {}", area);
                area
            }
            Shape::Circle(radius) => {
                let area: f64 = 3.14 * radius * radius;
                println!("Circle Area {}", area);
                area
            }
        }
    }
}

```
- Another Variation
```
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
```