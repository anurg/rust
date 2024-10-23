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
### Options/Result Enum (Provided by Rust)
Options - for Null Handling returns Some(T) or None

```
use std::ops::Index;

fn main() {
    let str: String = String::from(" Hello, How are you?");
    let position = find_first_a(str);
    match position {
        Some(i) => println!(" a found at {}", i),
        None => println!("a Not Found!"),
    }
}

fn find_first_a(str: String) -> Option<i32> {
    for (index, char) in str.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

```
- Custom Option Enum Example
```
enum CustomOption {
    Some(i32),
    None,
}
fn main() {
    let str: String = String::from(" Hello, How are you?");
    let position = find_first_a(str);
    match position {
        CustomOption::Some(i) => println!(" a found at {}", i),
        CustomOption::None => println!("a Not Found!"),
    }
}

fn find_first_a(str: String) -> CustomOption {
    for (index, char) in str.chars().enumerate() {
        if char == 'a' {
            return CustomOption::Some(index as i32);
        }
    }
    return CustomOption::None;
}

```
Result - for Error handling return ok(T) or Err(Error)
```
// Write a function that reads the content of a file.
use std::fs::read_to_string;
fn main() {
    let file = read_to_string("file.txt");
    match file {
        Ok(str) => println!("The content of file: {:?} ", str),
        Err(error_message) => println!("The Error Message {:?}", error_message),
    }
}
```

### Package Management
External packages are called crates.
```
cargo add chrono
```
```
use chrono::{Local, Utc};
fn main() {
    // UTC time
    let now = Utc::now();
    println!("The time right now is {} ", now);
    //local Time
    let local = Local::now();
    println!("The Local time now is {}", local);
    //Fomratted UTC Time
    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted Date & Time is {}", formatted);
}
```
### Memory Management

### Heaps & Stack
Stack is memeory where we keep static data - i.e. size of data does not inncrease
Heap is memory where we keep dynamic data - size of data can increase.
![rust_stack_heap](images/Rust_stack_heap.jpg)

![rust_stack_heap](images/Rust_stack_heap1.jpg)

### Mutability
- All variables in RUST are by default immutable.
- To make the variable mutable 
```
let mut var_temp:i32 = 10;
```
### No Return in function
If no return type is specified for a function, it returns an empty tuple, also known as a unit.
An empty tuple is represented by ().
```
fn make_nothing() -> () {
    return ();
}

// the return type is implied as ()
fn make_nothing2() {
    // this function will return () if nothing is specified to return
}

fn main() {
    let a = make_nothing();
    let b = make_nothing2();

    // Printing a debug string for a and b
    // Because it's hard to print nothingness
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);
}

```

### Functions returning multiple values
Functions can return multiple values by returning a tuple of values.Tuple elements 
can be referenced by their index number.Rust supports various kinds of destructuring 
, allowing us to extract sub-pieces of data structures in ergonomic ways.
```
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn main() {
    // return a tuple of return values
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // destructure the tuple into two variables names
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}
```

### Ownership, Moving, Borrowing, References
- In RUST, when you assign variable to another variable, or Pass it as function argument, then the ownership changes and the old variable is dropped off memory (Heap). This is called Moving.
- - Many variable can take immutable reference by using &var_name
- Only One can take mutable reference by using &mut var_name
- if mutable refernce is taken, then not even one can take immutable reference
- This is called Borrowing
- References must always be valid.
```
fn create_string() -> String {
    let mut s1 = String::from("Hello World!");
    let s2: &mut String = &mut s1;
    s2.push('A');
    s2.push('n');
    s2.push('u');
    s2.push('r');
    s2.push('a');
    s2.push('g');
    return s2.to_string();
}
fn main() {
    let s = create_string();
    println!("The returned String is : {}", s);
}
```
### Passing as function Argument (Code gives error, Borrow of moved value)
```
fn create_string() {
    let s1 = String::from("Hello World!");
    print_string(s1);
    println!("{}", s1);
}
fn print_string(s2: String) {
    println!("{}", s2)
}
fn main() {
    create_string();
}
```
- Correct Code
```
fn create_string() {
    let s1 = String::from("Hello World!");
    print_string(&s1);
    println!("{}", s1);
}
fn print_string(s2: &String) {
    println!("{}", s2)
}
fn main() {
    create_string();
}

```

### Vectors
- Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
- You can initialize vector by vec! (vector macro also)
- 

```
fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);
}
```
```
fn main() {
    let mut v = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // v.push(4);
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    println!("{:?}", v);
    println!("{:?}", even_filter(v));
}

fn even_filter(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }
    return new_vec;
}

```
- Another approach
```
fn main() {
    
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    println!("{:?}", v);
    println!("{:?}", even_filter(&mut v));
}

fn even_filter(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut i= 0;
    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        } else {
            i +=1;
        }
    }
    return vec;
}

```

### Hashmaps
- Hashmaps stores key value in Rust.

```
use std::collections::HashMap;
fn main() {
    let mut hmap = HashMap::new();
    hmap.insert("harkirat", 28);
    hmap.insert("Anurag", 50);
    let result = hmap.get("Anuraga");
    match result {
        Some(age)=> println!("{}",age),
        None=>println!("Not Found"),
    }
}
```
- Another way
```
fn main() {
    let pairs = vec![
        (String::from("harkirat"),28),
        (String::from("Anurag"),50)
        ];

    let grouped_pairs = group_value_by_key(pairs);
    for (key,val) in &grouped_pairs {
        println!("Key: {}, Value: {}",key,val);
    }
    println!("{:?}",&grouped_pairs);

}
```
```
use std::collections::HashMap;


fn group_value_by_key(pairs:Vec<(String,i32)>) -> HashMap<String,i32> {
    let mut hmap = HashMap::new();
    for (key,val) in pairs {
        hmap.insert(key, val);
    }
    return hmap;
}
fn main() {
    let pairs = vec![
        (String::from("harkirat"),28),
        (String::from("Anurag"),50)
        ];

    let grouped_pairs = group_value_by_key(pairs);
    for (key,val) in &grouped_pairs {
        println!("Key: {}, Value: {}",key,val);
    }
    println!("{:?}",&grouped_pairs);

}
```

### Iterators
```

fn main() {
    let nums= vec![1,2,3,4,5];
    for val in nums {
        println!("{:?}",val);
    }
}
```
```

fn main() {
    let nums= vec![1,2,3,4,5];
    let iter = nums.iter();
    for val in iter {
        println!("{:?}",val);
    }
    println!("{:?}",nums);
}
```
### Mutable Iterators (iter_mut)
```
fn main() {
    let mut nums= vec![1,2,3,4,5];
    let iter = nums.iter_mut();
    for val in iter {
        *val = *val * 2;
    }
    println!("{:?}",nums);
}
```
### Iterate using .next
```

```

