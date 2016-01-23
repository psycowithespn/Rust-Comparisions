fn main() {
    let x = 5;
    let y = &x;
    let mut z = &x;
    *z = *z + 1;
    println!("{}", z);
}