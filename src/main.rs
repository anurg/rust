fn main() {
    let s3 = String::from("Hello World!");
    {
        let s4 = &s3;

        println!("{}", s4);
    }
    println!("{}", s3);
}
