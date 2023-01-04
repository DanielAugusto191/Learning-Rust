use std::{collections::HashMap, task::Context};

#[derive(Debug, Clone)]
enum Term {
    Var(String),
   Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}
fn eval(term: &Term, mut context: &mut HashMap<String, Term>) -> Term {
    match term {
        Term::Var(X) => {
            let b = context.get(X);
            match b {
                Some(X) => X.clone(),
                None => Term::Var(X.to_string()),
            }
        }
        Term::Abs(X, Y) => Term::Abs(X.clone(), Box::new(eval(Y, context))),
        Term::App(X, Y) => {
            let eval_func = eval(X, &mut context);
            let eval_agr = eval(Y, &mut context);
            println!("{:?}", eval_func);
            match eval_func {
                Term::Abs(X, Y) => {
                    context.insert(X.clone(), eval_agr);
                    let res = eval(&Y, context);
                    context.remove(&X);
                    res
                }
                _ => panic!("?"),
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
Box::new(Term::App(Box::new(Term::Var("n".to_string())), Box::new(Term::Var("f".to_string())))),
                        Box::new(Term::Var("x".to_string())),
                    )),
                )),
            )),
        )),
    );
    let one = Term::App(Box::new(succ_.clone()), Box::new(zero_.clone()));
    let l = Term::Var("X".to_string());
    let p = Term::App(Box::new(Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string())))), Box::new(Term::Var("k".to_string())));
    let mut context = HashMap::new();
    let res = eval(&zero_, &mut context);
//    let res = eval(&zero_, &mut context);
   println!("{:?}", res);
}
