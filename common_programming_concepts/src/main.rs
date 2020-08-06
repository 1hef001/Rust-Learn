fn main(){
    println!("This is main function");
    let y = {
        let x = 1;
        x + 2
    };
    println!("The value of y is {}", y);
}