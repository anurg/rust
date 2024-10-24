fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
    let iter_mut = vec.iter_mut();

    for val in iter_mut {
        println!("{}", val);
        *val = *val * 3;
        println!("{}", val);
    }
    println!("{:?}", vec);
}
