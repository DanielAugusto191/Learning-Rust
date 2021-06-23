// Expressions return values
// Statements do not return values

fn main() {
    new_func(5, 6);
    println!("Hello, world!");
    let _x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    println!("{}", plus1(4));
    println!("{}", plus2(4));
}
fn new_func(x: i32, y: i32){
    println!("{}, {}", x, y);
}

fn plus1(x: i32) -> i32{
    x+1
}

fn plus2(x: i32) -> i32{
    return x+2;
}