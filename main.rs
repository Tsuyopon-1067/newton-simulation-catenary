fn main() {
    let l = 48.0;
    let h = 14.0;
    let b = 4.0*h/l;
    println!("l={}, h={}, b={}", l, h, b);
    let u = newton(100.0, b);
    let a = l / (2.0*u);
    println!("a = {}", a);
}

fn f(x: f64, b: f64) -> f64 {
    let e = std::f64::consts::E;
    return e.powf(2.0*x) - 2.0*e.powf(x) + 1.0 - b*x*e.powf(x);
}
fn df(x: f64, b: f64) -> f64 {
    let e = std::f64::consts::E;
    return 2.0*e.powf(2.0*x) - 2.0*e.powf(x) - b*(x+1.0)*e.powf(x);
}
fn newton(mut x: f64, b:f64) -> f64 {
    let delta = 1e-12;
    let mut k = 0;
    while f(x, b) > delta {
        if df(x, b) == 0.0 {
            println!("df(x) = 0");
            println!("x = {}", x);
            break;
        }
        x = x - f(x, b) / df(x, b);
        println!("{} : x = {}, f(x) = {}, f'(x) = {}", k, x, f(x, b), df(x, b));
        k += 1;
    }
    return x;
}