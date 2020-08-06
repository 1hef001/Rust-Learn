fn another_fn(x: (i32, u128, f64)){
    println!("This is another function");
    println!("And the value of x(0) is {}", x.0);
}

fn main(){
    println!("This is main function");
    another_fn((-5, 8, 6.4));
}