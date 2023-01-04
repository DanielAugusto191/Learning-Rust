use std::{collections::HashMap, task::Context};

#[derive(Debug, Clone)]
enum Term {
    Var(String),
   Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

enum Value {
    Closure(HashMap<String, Term>, String, Term),
}

fn eval(term: &Term, mut context: &mut HashMap<String, Term>) -> Term {
    //println!("{:?}", term);
    match term {
        Term::Var(X) => {
            let b = context.get(X);
            match b {
                Some(x) => x.clone(),
                None => Term::Var(X.clone())
            }
        }
        Term::Abs(X, Y) => Term::Abs(X.clone(), Box::new(eval(&Y, context))),
        Term::App(X, Y) => {
            let funct = eval(&X, context);
            let args = eval(&Y, context);
           println!("{:?}", funct);
           println!("{:?}", args);
            match funct{
                Term::Abs(A, B) => {
 //                   println!("{:?}", B);
                    context.insert(A.clone(), args);
                    let res = eval(&B, context);
                    context.remove(&A);
                    res
                }
                _ => panic!("Not a abs"),
            }
        }
    }
}

fn true_<T>(f: fn(T) -> T, x: T) -> T {
    f(x)
}
fn false_<'a, T>(f: Box<dyn Fn(&'a T) -> &'a T>, x: &'a T) -> &'a T {
    x
}
fn not<'a, T>(a: Box<dyn Fn(&'a T) -> T + 'a>) -> Box<dyn Fn(&'a T) -> T + 'a> {
    Box::new(move |x: &'a T| a(false_(Box::new(move |y: &'a T| y), x)))
}

fn _T<'a, T>(
    x: Box<dyn Fn(&'a T, &'a T) -> &'a T>,
    y: Box<dyn Fn(&'a T, &'a T) -> &'a T>,
) -> Box<dyn Fn(&'a T, &'a T) -> &'a T> {
    x
}
fn _F<'a, T>(x: &'a T, y: &'a T) -> &'a T {
    y
}
// fn _and<'a, T>(x: Box<dyn Fn(Box<dyn Fn(&'a T,&'a T) -> &'a T>,Box<dyn Fn(&'a T,&'a T) ->&'a T>) -> &'a T>, y: Box<dyn Fn(&'a T,&'a T) -> &'a T>) -> &'a T {
//     x(y, Box::new(_F))
// }
fn zero<T>(x: T, y: T) -> T {
        y
}
fn one<T>(x: fn(T) -> T, y: T) -> T {
    x(y)
}
fn two<T>(x: fn(T) -> T, y: T) -> T {
    x(x(y))
}
fn succ<T>(w: fn(T) -> T, y: fn(T) -> T, x: T) -> T {
    y(w(y(x)))
}

pub fn main() {
    let k = Term::Var("LIMAO".to_string());
    let zero_ = Term::Abs(
        "s".to_string(),
        Box::new(Term::Abs(
            "z".to_string(),
            Box::new(Term::Var("z".to_string())),
        )),
    );
    let succ_ = Term::Abs(
        "n".to_string(),
        Box::new(Term::Abs(
            "f".to_string(),
            Box::new(Term::Abs(
                "x".to_string(),
                Box::new(Term::App(
                    Box::new(Term::Var("f".to_string())),
                    Box::new(Term::App(
                        Box::new(Term::App(
                            Box::new(Term::Var("n".to_string())),
                            Box::new(Term::Var("f".to_string())))),
                        Box::new(Term::Var("x".to_string())),
                    )),
                )),
            )),
        )),
    );
    let one = Term::App(Box::new(succ_.clone()), Box::new(zero_.clone()));
    let l = Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string())));
    let p = Term::App(Box::new(l.clone()), Box::new(Term::Var("k".to_string())));

    let TT = Term::Abs("x".to_string(), Box::new(Term::Abs("y".to_string(), Box::new(Term::Var("x".to_string())))));
    let and_ = Term::Abs(
        "p".to_string(),
        Box::new(Term::Abs(
                "q".to_string(),
                Box::new(Term::App(
                        Box::new(Term::App(
                                Box::new(Term::Var("p".to_string())),
                                Box::new(Term::Var("q".to_string()))
                                )), 
                        Box::new(Term::Var("p".to_string()))
                        ))
                ))
        );
    let TTT = Term::App(
        Box::new(Term::App(
            Box::new(and_.clone()),
            Box::new(TT.clone())
        )),
        Box::new(TT.clone())
    );
    let identy = Term::Abs(
            "x".to_string(),
            Box::new(Term::Var("x".to_string()))
        );
    let appIdenty = Term::App(
            Box::new(identy.clone()),
            Box::new(identy.clone())
        );
    let mut context = HashMap::new();
    let res = eval(&TTT, &mut context);
//    let res = eval(&zero_, &mut context);
   println!("{:?}", res);
}
