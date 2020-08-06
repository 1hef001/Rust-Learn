fn main() {
    let tup: (i32, f64, u8) = (-4, 2.8, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is {}", y);
    println!("The first value in the tuple is: {}", tup.0)
}
