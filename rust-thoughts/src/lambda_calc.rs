enum calc {
    Var(String),
    Application(Box<calc>, Box<calc>),
    Abstraction(Box<String>, Box<calc>),
}

pub fn main() {
    let b = calc::Abstraction;
}
