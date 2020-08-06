use std::io;

fn fibo_gen(lim: u32){

    if lim > 3 {
        let mut a = 0;
        let mut b = 0;
        let mut c = 1;
        println!("{} ", a);
        println!("{} ", b);
        // println!("{} ", c);

        for _ in 2..lim {
            println!("{}", c);
            a = b;
            b = c;
            c = a + b;
        }
    }
}

fn main(){
    // let a = [1, 2, 3, 4, 5];
    let mut limit = String::new();
    io::stdin()
        .read_line(&mut limit)
        .expect("Failed to read message");

    let limit: u32 = limit.trim().parse().expect("Enter Appropriate number");
    fibo_gen(limit);
}