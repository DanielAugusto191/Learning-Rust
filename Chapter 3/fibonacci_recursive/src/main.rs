use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid Input");
    let input: i32 = input.trim().parse().unwrap();
    println!("{}", fib(input));
}

fn fib(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return 1;
    }
    fib(n-1) + fib(n-2)
}
