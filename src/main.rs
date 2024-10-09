fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn main() {
    let my_string = String::from("Testing Rust");
    let index = find_first_a(my_string);
    match index {
        Some(value) => println!("The index is {}", value),
        None => println!("a not found!"),
    }
}
