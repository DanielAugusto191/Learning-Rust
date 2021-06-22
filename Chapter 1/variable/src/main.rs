fn main() {
    let mut x = 5;
    println!("The value of X is: {}", x);
    x = 6;
    println!("The value of X is: {}", x);

    let y = 5;
    println!("The value of Y is: {}", y);
    let y = 2*y; // Shadowing y = 5;
    println!("The value of Y is: {}", y);

    let spaces = "   ";
    let _spaces = spaces.len();

    // let mut spaces2 = "   "; 
    // spaces2 = spaces2.len(); Missmatch Types

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;

    // remainder
    let _remainder = 43 % 5;

    let _c = 'z';
    let _z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // Pattern matching to desctructure a tuple 

    println!("The value of x,y and z is: {},{},{} respectively", x,y,z);
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;

    let _months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5]; // 3 3 3 3 3
}
