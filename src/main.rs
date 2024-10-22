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

### Memory Management
