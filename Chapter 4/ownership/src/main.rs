fn main() {
    let _a = "test"; // On stack
    let mut _str = String::from("hello"); // On Heap, memory needs to be release.
    _str.push_str(", world!");

    let _x = 5;
    let _y = _x; // Copy on stack
    println!("{}, {}", _x, _y);

    let s1 = String::from("Hello");
    let _s2 = s1; // Pointer for the same heap.
    // println!("{}, world", s1); s1 is no more avalible.

    let s1 = String::from("Hello");
    let _s2 = s1.clone(); // s2 copy of s1;
    println!("{}, {}", s1, _s2);

    // Functions
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    let _s = gives_ownership();
    let s2 = String::from("Hello");
    let _s2 = takes_and_gives_back(s2);
    let (s3, len) = calc_len(_s2);
    println!("{}, length: {}", s3, len);

    // References
    let len = calc_len_reference(&s3);
    println!("{}, lenght: {}", s3, len);

    let mut s4 = String::from("hello");
    modify_reference(&mut s4);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}
fn takes_and_gives_back(s1: String) -> String{
    s1
}

fn calc_len(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}

fn calc_len_reference(s: &String) -> usize{
    s.len()
}

fn modify_reference(s: &mut String){
    s.push_str("test");
}