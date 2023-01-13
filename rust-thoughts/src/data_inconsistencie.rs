use std::rc::Rc;
use std::cell::RefCell;
pub fn main(){
    let mut a = Rc::new(RefCell::new("Limao")); 
    let b = Rc::clone(&a);
    let mut c = Rc::clone(&a);

    println!("Pre Change B\n{:?}", b);
    println!("{:?}", c);
    
    *b.borrow_mut() = "Laranjas";
    println!("Pos Change B\n{:?}", b);
    println!("{:?}", c);
    
    *c.borrow_mut() = "Kiwi";
    println!("Pos Change C\n{:?}", b);
    println!("{:?}", c);
}
