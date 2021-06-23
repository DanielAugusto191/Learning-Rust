fn main() {
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10 {
            break counter * 2; // Break loop, and return value;
        }
    };
    println!("Result 1: {}", result);

    counter = 3;
    while counter != 0 {
        counter -= 1;
    }
    println!("Result 2: {}", counter);

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
    
    let a = 1..5;
    for element in a {
        println!("the value is: {}", element);
    }
    
    for number in (1..4).rev() {
        println!("{}!", number);
    }


}
