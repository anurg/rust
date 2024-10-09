use std::fs;

fn main() {
    let contents_of_file = fs::read_to_string("hello.txt");
    let ans = match contents_of_file {
        Ok(file_content) => {
            println!("The contents of file {:?} ", file_content)
        }
        Err(error) => {
            println!(" Failed to Read File! {:?}", error)
        }
    };
    return ans;
}
