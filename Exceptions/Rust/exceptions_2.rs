fn main() {
    let number = "42";
    let result : i32 = number.parse();
    println!("{} + 10 = {}", number, result + 10);
    
    let not_a_number = "asdf";
    let not_a_result : i32 = not_a_number.parse().unwrap();
    println!("{} + 10 = {}", not_a_number, not_a_result + 10);
}