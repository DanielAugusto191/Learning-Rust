#![allow(warnings)]
use core::panic;
use std::fmt::Debug;

enum Bool_ {
    True,
    False,
    name(i32),
}

enum skin_color {
    Black,
    White,
}

enum List<T: Debug> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T: Debug> List<T> {
    fn print(&self) {
        match self {
            Nil => return,
            Cons(X, Y) => {
                println!("{:?}", X);
                Y.print()
            }
        }
    }
}

struct Person {
    name: String,
    age: i32,
    skin_color: skin_color,
}

#[derive(Debug)]
enum Food {
    Rice,
    Beans,
}

fn c1(food: Food) -> Option<Food> {
    match food {
        Food::Rice => None,
        _ => Some(food),
    }
}

fn invert(food: Food) -> Option<Food> {
    match food {
        Food::Rice => Some(Food::Beans),
        Food::Beans => Some(Food::Rice),
    }
}

enum MyOption<T> {
    None,
    Some(T),
}

impl Food {
    fn teste(&self) {
        println!("?");
    }
}

mod church_encoding;
mod lambda_calc;
mod data_inconsistencie;
struct limao(pub i32, pub Option<&'static str>);
use List::{Cons, Nil};
use std::collections::HashMap;
fn f(a: &mut HashMap<String, i32>) {
    let aux = a.get(&"a".to_string());
    match aux {
        Some(x) => println!("{}", x),
        None => panic!("?")
    }
    a.insert("b".to_string(), 20);
    println!("{}", a.len());
}

fn id<T>(x: T) -> T{
    x
}

fn main() -> Result<(), ()> {
    // let b = limao(12, Some("ok"));
    // println!("{}", b.0);
    // println!("{}", b.1.unwrap());
    // let k: i32 = Default::default();
    // println!("{}", k);

    // let b: List<i32> = List::Cons(50, Box::new(List::Cons(51, Box::new(Nil))));
    // b.print();
  //  lambda_calc::main();
 //   church_encoding::main();
        data_inconsistencie::main();
//    teste::main();
    println!("{:?}", id(true));
    Ok(())
}
