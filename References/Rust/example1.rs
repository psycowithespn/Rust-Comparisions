use std::str::Chars;

fn main() {
    let x = "Hello World".chars();
    let y = copy(x);
    println!("x: {}, y: {}", x.as_str(), y.as_str());
}

fn copy(x: Chars) -> Chars {
    x.clone()
}