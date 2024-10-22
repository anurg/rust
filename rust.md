### Rust Notes

### Rust Variables

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

