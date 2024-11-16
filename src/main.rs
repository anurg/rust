use std::fs::read_to_string;
use std::io::Error;
use std::path::Path;
fn read_file_content(file: &Path) -> Result<String, Error> {
    let result = read_to_string(file)?;
    Ok(result)
}
fn read_file_content_unwrap(file: &str) -> String {
    read_to_string(file).unwrap()
}
fn main() {
    let file_path = Path::new("./src/file.txt");
    match read_file_content(file_path) {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error Message: {}", e),
    }
    println!("{}", read_file_content_unwrap("file.txt"));
    println!("Current Working Directory: {:?} ", std::env::current_dir());
}
