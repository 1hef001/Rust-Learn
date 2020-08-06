fn ret() -> i32 {
    -5
}

fn main(){
    println!("This is main function");
    let y = ret();
    println!("The value of y is {}", y);
}