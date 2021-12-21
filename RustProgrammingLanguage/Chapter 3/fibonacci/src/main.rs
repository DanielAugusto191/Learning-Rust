use std::io;

fn main() {
    println!("N-th fib sequence: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid Input");
    let input: i32 = input.trim().parse().unwrap();
    if input == 1 || input == 2 {
        println!("1");
    }else{
        let mut a = 2;
        let mut b = 1;
        for _ in 3..input {
            b += a;
            a += b;
            b = a-b;
            a = a-b;
        }
        println!("{}", a);
    }
}
