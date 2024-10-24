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
