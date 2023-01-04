use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Term {
    Var(String),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

struct Interpreter {
    store: HashMap<String, Term>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            store: HashMap::new(),
        }
    }

    fn eval(&mut self, t: &Term) -> Term {
        match t {
            Term::Var(x) => self.store.get(x).unwrap().clone(),
            Term::Abs(x, t1) => Term::Abs(x.to_string(), Box::new(self.eval(t1))),
            Term::App(t1, t2) => {
                let t1 = self.eval(t1);
                let t2 = self.eval(t2);
                match t1 {
                    Term::Abs(x, t12) => {
                        self.store.insert(x.to_string(), t2);
                        let res = self.eval(t12);
                        self.store.remove(x);
                        res
                    }
                    _ => panic!("Cannot apply non-function"),
                }
            }
        }
    }
}

fn main() {
    let mut interpreter = Interpreter::new();

    // the Church-encoded successor function
    let succ = Term::Abs(
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

    // apply the successor function to the Church-encoded number 5
    let t = Term::App(
        Box::new(succ),
        Box::new(Term::Abs(
            "f".to_string(),
            Box::new(Term::Abs(
                "x".to_string(),
                Box::new(Term::App(
                    Box::new(Term::App(
                        Box::new(Term::App(
                            Box::new(Term::App(
                                Box::new(Term::App(
                                    Box::new(Term::Var("f".to_string())),
                                    Box::new(Term::Var("f".to_string())),
                                )),
                                Box::new(Term::Var("f".to_string())),
                            )),
                            Box::new(Term::Var("f".to_string())),
                        )),
                        Box::new

