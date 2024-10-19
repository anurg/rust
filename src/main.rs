fn main() {
    let num: i32 = 41;
    for i in 0..num {
        println!("The number: {} ", i);
    }
    println!("Hello Rustacean {}", "Anurag");
    println!("The result: {} {}", num, is_even(num));
}

fn is_even(num: i32) -> String {
    if num % 2 == 0 {
        return String::from("is Even");
    } else {
        return String::from("is not Even");
    }
}

fn get_str_len(str: String) -> usize {
    return str.chars().count();
}
