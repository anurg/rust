fn main() {
    let str:String = String::from("Hello New AI World!");
    let n = get_string_length(str);
    println!("The length of string is {}",n);
}

fn get_string_length(str:String) -> usize {
    return str.chars().count();
}