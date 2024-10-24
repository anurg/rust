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
