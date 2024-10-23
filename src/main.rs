
fn main() {
    let mut nums= vec![1,2,3,4,5];
    let iter = nums.iter_mut();
    for val in iter {
        *val = *val * 2;
    }
    println!("{:?}",nums);
}