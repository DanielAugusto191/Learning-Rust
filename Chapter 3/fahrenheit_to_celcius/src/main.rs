use std::io;

fn main() {
    println!("Fahrenheit to Celcius");
    loop{
        let mut input = String::new();
        print!("Value in Fahrenheit: ");
        io::stdin().read_line(&mut input).expect("Not valid input");
        let input: f32 = input.trim().parse().unwrap(); 
        let ans: f32 = (input-32.0) * 5.0/9.0;
        println!("{} ºF is {} ºC", input, ans);
    }
}
