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

### Reference & Dereference
- Dereferencing
The process of accessing/manipulating data that is being referred to by a reference (i.e. &i32) is called dereferencing.

References are used to access/manipulate data in two ways:

Access to the referred data during assignment of variables.
Access to fields or methods of the referred data.
Rust has some powerful operators that allow us to do this.
- The & operator (reference):
Creates a reference/pointer to a value
Allows borrowing without taking ownership

```
let x = 5;          // Regular variable
let y = &x;         // y is a reference to x
println!("x: {}, y: {}", x, *y);  // Need to dereference y to get its value
```
### Question- In this example, if I remove dereference, still y prints the value as 5 so whats the use of *?     
 - this is a common source of confusion in Rust! The reason you can print y without an explicit dereference is because Rust's println! macro and the Display/Debug traits handle references automatically through a feature called "auto-dereferencing." However, this automatic behavior doesn't work in all contexts.
Let me demonstrate when you actually need the explicit dereference (*):
```
let x = 5;
let y = &x;

// These both print "5" due to println!'s automatic dereferencing
println!("x: {}, y: {}", x, y);
println!("x: {}, y: {}", x, *y);

// But in these cases, you MUST use explicit dereferencing:

// 1. Arithmetic operations
let z = *y + 2;  // Works
// let z = y + 2;  // Error! Can't add a reference to an integer

// 2. Comparisons with non-reference values
if *y == 5 { }  // Works
// if y == 5 { }  // Error! Can't compare reference with integer

// 3. Assigning to new variables
let a: i32 = *y;  // Works
// let a: i32 = y;  // Error! Can't assign reference to i32

// 4. Function calls that expect the actual value
fn takes_i32(num: i32) { }
takes_i32(*y);  // Works
// takes_i32(y);  // Error! Function expects i32, not &i32
```
###  Think of it like this:
- y is like an address/pointer to where x is stored
- *y is explicitly saying "go to that address and get me the value"
- Some Rust features (like println!) automatically follow that address for convenience
- But in most other cases, you need to be explicit about following the address with *

- The * operator (dereference):
Accesses the value a reference points to
"Follows" the pointer to get the actual value
```
let a = 10;         // Regular variable
let b = &a;         // Reference to a
let c = *b;         // Dereference b to get a's value
println!("c: {}", c); // Prints: 10
```

### Iterators
- Iterating using for loops
```
fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7];
    for val in vec {
        println!("{}", val); //`vec` moved due to this implicit call to `.into_iter()`
    }
    // println!("{:?}", vec);
}
```
- Iterating after creating an Iterator
- Iterators are lazy. Just defining them has no effect until you call the methods which consume the iterator.
- iter methos iterates over a collection by Borrowing them.
- You cannot mutate the elements as you have immutable reference to internal elements.
```
fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7];
    let iter = vec.iter();
    for val in iter {
        println!("{}", val); 
    }
    println!("{:?}", vec);
    // for new_val in iter {   //use of moved value: `iter`
    //     println!("{}", new_val);
    // }
}
```
### Mutable Iterators (IterMut)
```
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
    let iter_mut = vec.iter_mut();

    for val in iter_mut {
        println!("{}", val);
        *val = *val * 3;
        println!("{}", val);
    }
    println!("{:?}", vec);
}
```
### Iterate using .next
```
fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7];
    let mut iter = vec.iter();
    while let Some(val) = iter.next() {
        println!("{}", val);
    }
}
```
### The while let pattern is particularly useful when you need to:

- Process a sequence of values
- Don't know how many items you'll process in advance
- Want to stop when a certain pattern no longer matches
- Want cleaner code than a loop with match
### Alternative to While..let
```
fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7];
    let mut iter = vec.iter();

    // while let Some(val) = iter.next() {
    //     println!("{}", val);
    // }
    loop {
        match iter.next() {
            Some(val) => println!("{}", val),
            None => break,
        }
    }
}
```
### into_iter
- This iterator takes the ownership of the collection.
- It is use when you don't need original collection 
- Also it is useful when you need to squeeze the performance benefits by transfering ownership(by avoiding reference.)
```
fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7];
    for val in vec.into_iter() {
        println!("{}", val);
    }
    // println!("{:?}", vec); //borrow of moved value:
}
```
### Difference in Iterators
![Iterators_Image](images/iterators.jpg)

### Consuming Adapters
- methods are called consuming adapters because calling them uses up the iterator.You cannot use iterator again. 
- Doesn't consume the collection
```
fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let v1_iter = vec.iter();

    let total: i32 = v1_iter.sum();
    println!("The total is : {}", total);
    println!("{:?}", vec);
    // let sum1: i32 = v1_iter.sum(); 
}
```

### Iterative Adapters
- methods called on iterators which does not consume iterators but produce different iterators by changing some aspect of the original adapter.
### map
```
fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let v1_iter = vec.iter();
    let v2_iter = v1_iter.map(|x| *x * 2);
    for val in v2_iter {
        println!("{}", val);
    }
    println!("{:?}", vec);
    // let sum1: i32 = v1_iter.sum();
}
```
### filter
```
fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7];
    let iter = vec.iter();
    let iter1 = iter.filter(|x| **x % 2 != 0);
    for val in iter1 {
        println!("{}", val);
    }
}
```
### Chaining 
```
fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7];
    let iter = vec.iter();
    let iter1 = iter.filter(|x| **x % 2 != 0).map(|x| *x * 2);
    for val in iter1 {
        println!("{}", val);
    }
}
```
### Collecting back the values from Iterator to Vector
```
fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7];
    let iter = vec.iter();
    let iter1 = iter.filter(|x| **x % 2 != 0).map(|x| *x * 2);
    // for val in iter1 {
    //     println!("{}", val);
    // }
    let x: Vec<i32> = iter1.collect();
    println!("{:?}", x);
}
```
### Iterators in HashMaps
```
use std::collections::HashMap;
fn main() {
    let mut hm = HashMap::new();
    hm.insert("Person1", 50);
    hm.insert("Person2", 25);
    hm.insert("Person3", 30);
    println!("{:?}", hm);
    // Example1- Iterating over hashmap
    // for (ky, vl) in hm {
    //     println!("Key: {}, Value: {}", ky, vl);
    // }
    //Example2- Iterating over mutable reference
    for (key, val) in hm.iter_mut() {
        *val = *val + 5;
    }
    println!("{:?}", hm);
}
```

### Strings & Slices

```
fn main() {
    let str = String::from("Anurag Bhatt");
    let result = first_word(str);
    println!("{}", result);
}

fn first_word(str: String) -> String {
    let mut new_str = String::from("");
    for char in str.chars() {
        if char == ' ' {
            break;
        }
        new_str.push(char);
    }
    return new_str;
}
```
- The problem with this code is that it takes double the memory for str & new_str variables.
- If the str String gets cleared, still new_str String has "Anurag" value in it.
- What we want is the View over original String and not copy it over.
- Modified code with slices
```
fn main() {
    let str = String::from("Anurag Bhatt");
    let result = find_first_word(&str);
    println!("{}", result);
}

fn find_first_word(my_str: &String) -> &str {
    let mut index = 0;
    for (i, char) in my_str.chars().enumerate() {
        if char == ' ' {
            break;
        }
        // index = index + 1;
        index = i;
    }
    return &my_str[0..index];
}
```