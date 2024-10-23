use std::collections::HashMap;


fn group_value_by_key(pairs:Vec<(String,i32)>) -> HashMap<String,i32> {
    let mut hmap = HashMap::new();
    for (key,val) in pairs {
        hmap.insert(key, val);
    }
    return hmap;
}
fn main() {
    let pairs = vec![
        (String::from("harkirat"),28),
        (String::from("Anurag"),50)
        ];

    let grouped_pairs = group_value_by_key(pairs);
    for (key,val) in &grouped_pairs {
        println!("Key: {}, Value: {}",key,val);
    }
    println!("{:?}",&grouped_pairs);

}