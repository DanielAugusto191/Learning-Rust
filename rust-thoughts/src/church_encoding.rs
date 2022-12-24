fn true_<T>(f: fn(T) -> T, x: T) -> T {
    f(x)
}
fn false_<'a, T>(f: Box<dyn Fn(&'a T) -> &'a T>, x: &'a T) -> &'a T {
    x
}
fn not<'a, T>(a: Box<dyn Fn(&'a T) -> T + 'a>) -> Box<dyn Fn(&'a T) -> T + 'a> {
    Box::new(move |x: &'a T| a(false_(Box::new(move |y: &'a T| y), x)))
}

pub fn main(){
    let a = 12;
    let b = |x: i32, y: i32| x + y;
}