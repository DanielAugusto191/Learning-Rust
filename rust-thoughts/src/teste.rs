use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Term {
    Var(String),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>)
}
pub fn main() {
    // Define the lambda term for the succ function: \n.\s.\z.s (n s z)
    let succ = Term::Abs(
        "n".to_string(),
        Box::new(Term::Abs(
            "s".to_string(),
            Box::new(Term::Abs(
                "z".to_string(),
                Box::new(Term::App(
                    Box::new(Term::App(
                        Box::new(Term::Var("s".to_string())),
                        Box::new(Term::Var("n".to_string())),
                    )),
                    Box::new(Term::Var("z".to_string())),
                )),
            )),
        )),
    );

    // Define the lambda term for the zero function: \s.\z.z
    let zero = Term::Abs(
        "s".to_string(),
        Box::new(Term::Abs(
            "z".to_string(),
            Box::new(Term::Var("z".to_string())),
        )),
    );

    // Define the lambda term for the one function: succ zero
    let one = Term::App(Box::new(succ.clone()), Box::new(zero.clone()));

    // Define the lambda term for the two function: succ one
    let two = Term::App(Box::new(succ.clone()), Box::new(one.clone()));

    // Define the lambda term for the three function: succ two
    let three = Term::App(Box::new(succ.clone()), Box::new(two.clone()));

    // Evaluate the three function
    let mut env = HashMap::new();
    let result = eval_cbv(&three, &mut env);

    println!("Result: {:?}", result);
}

fn eval_cbv(term: &Term, env: &mut HashMap<String, Term>) -> Option<Term> {
    match term {
        Term::Var(name) => {
            // Look up the variable in the environment
            env.get(name).cloned()
        }
        Term::Abs(arg, body) => {
            // Create a closure and return it
            Some(Term::Abs(arg.clone(), Box::new(eval_cbv(body, env)?)))
        }
        Term::App(func, arg) => {
            // Evaluate the function and argument
            let func_val = eval_cbv(func, env)?;
            let mut arg_env = env.clone();
            let arg_val = eval_cbv(arg, &mut arg_env)?;

            match func_val {
                Term::Abs(name, body) => {
                    // Bind the argument to the parameter and evaluate the body
                    env.insert(name.clone(), arg_val);
                    let result = eval_cbv(&body, env);
                    env.remove(&name);
                    result
                }
                _ => None,
            }
        }
    }
}
