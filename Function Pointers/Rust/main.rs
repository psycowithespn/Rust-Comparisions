fn main() {
    let mut op: fn(f64, f64) -> f64 = add;
    let resultA = op(10.0, 20.0);
    println!("{}", resultA); // Prints '30'
    
    op = subtract;
    let resultB = op(10.0, 20.0);
    println!("{}", resultB); // Prints '-10'
}

fn add(a : f64, b: f64) -> f64 {
    a + b
}

fn subtract(a : f64, b: f64) -> f64 {
    a - b
}