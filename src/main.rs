
fn main() {
    let nums= vec![1,2,3,4,5];
        let mut nums_iter = nums.iter();
        while let Some(val) = nums_iter.next() {
            println!("{}",val);
        }
        println!("{:?}",nums);
    }
    