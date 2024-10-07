fn main() {
    //     let _sentence: String = String::from("Hello World!, Here I come");
    //     let first_word = get_first_word(_sentence);
    //     let n = 1000;
    //     for i in 0..n {
    //         println!("Loop No: {}", i);
    //     }
    //     println!("The first word is : {} ", first_word);
    // }

    // pub fn get_first_word(my_str: String) -> String {
    //     let mut ans: String = String::from("");
    //     for char in my_str.chars() {
    //         // ans.push(char);
    //         ans.push_str(char.to_string().as_str());
    //         if char == ' ' {
    //             break;
    //         }
    //     }
    //     return ans;
    let a = 5;
    let b = 10;
    println!("The sum of {} and {} is {}", a, b, do_sum(a, b));
}

pub fn do_sum(a: i32, b: i32) -> i32 {
    return a + b;
}
