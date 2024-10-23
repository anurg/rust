fn main() {
    let mut v = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // v.push(4);
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    println!("{:?}", v);
    println!("{:?}", even_filter(v));
}

fn even_filter(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }
    return new_vec;
}
