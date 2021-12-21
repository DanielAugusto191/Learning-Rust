fn main() {
    let s = String::from("sweet ");
    let k = first_word(&s[..]);
    println!("{}", k);
    let x = second_word(&s[..]);
    println!("{}", x);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { 
            return &s[0..i]; 
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut k = 0;
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            if k != 0 {
                return &s[k+1..i];
            }
            k = i;
        }
    }
    if k != 0 {
        return &s[k+1..];
    }
    ""
}